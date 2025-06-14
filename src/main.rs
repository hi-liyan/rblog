use anyhow::Result;
use clap::Parser;
use gray_matter::engine::TOML;
use gray_matter::Matter;
use models::{Config, FrontMatter, Post};
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};
use walkdir::WalkDir;

mod models;

// 使用 clap 定义命令行接口
#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Builds the static site
    Build,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build => {
            build_site()?;
        }
    }
    Ok(())
}

fn build_site() -> Result<()> {
    println!("Building site...");

    // 1. 清理并创建输出目录
    let output_dir = Path::new("public");
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir(output_dir)?;

    // 2. 加载配置
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;

    // 3. 解析所有Markdown文章
    let mut posts = Vec::new();
    for entry in WalkDir::new("content").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "md" {
            let file_content = fs::read_to_string(path)?;

            // 解析Front Matter
            let matter = Matter::<TOML>::new();
            let result = matter.parse(&file_content);
            let front_matter: FrontMatter = result.data.unwrap().deserialize()?;

            // 解析Markdown内容
            let options = Options::empty();
            let parser = MarkdownParser::new_ext(&result.content, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            // 生成永久链接
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let permalink = format!("/{}/", file_stem);

            posts.push(Post {
                metadata: front_matter,
                content_html: html_output,
                permalink,
            });
        }
    }

    // 按日期排序文章
    posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

    // 4. 初始化模板引擎
    println!("Loading templates...");
    let mut tera = Tera::default();
    let templates_path = format!("themes/{}/templates", config.theme);

    // 我们需要手动收集所有模板
    let mut templates_to_add = Vec::new();
    for entry in WalkDir::new(&templates_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "html" {
            // 获取相对于模板目录的路径，作为模板的名称
            let relative_path_ref = path.strip_prefix(&templates_path)?.to_str().unwrap();
            let relative_path = relative_path_ref.to_string();
            let content = fs::read_to_string(path)?;
            templates_to_add.push((Some(relative_path), content));
            println!("  - Found template: {}", relative_path_ref);
        }
    }

    // 将模板内容转换为 &str，因为 add_raw_templates 需要它
    let templates_str_refs: Vec<(&str, &str)> = templates_to_add
        .iter()
        .map(|(path, content)| (path.as_deref().unwrap(), content.as_str()))
        .collect();

    // 一次性添加所有模板，让 tera 自己处理依赖关系
    tera.add_raw_templates(templates_str_refs)?;
    println!("Templates loaded and parsed successfully!");

    // 5. 渲染页面
    println!("\nRendering pages...");
    // 渲染单篇文章
    for post in &posts {
        println!("  -> Rendering post: '{}'", post.metadata.title);

        let mut context = Context::new();
        context.insert("config", &config);
        context.insert("post", &post);

        let rendered_html = tera.render("post.html", &context)?;

        let post_dir = output_dir.join(post.permalink.trim_start_matches('/'));
        fs::create_dir_all(&post_dir)?;
        let mut file = File::create(post_dir.join("index.html"))?;
        file.write_all(rendered_html.as_bytes())?;
    }

    // 渲染首页
    println!("\nRendering index page...");

    let mut index_context = Context::new();
    index_context.insert("config", &config);
    index_context.insert("posts", &posts);
    let index_html = tera.render("index.html", &index_context)?;
    let mut index_file = File::create(output_dir.join("index.html"))?;
    index_file.write_all(index_html.as_bytes())?;

    // 6. 复制CSS和静态文件
    println!("Copying static files...");
    let style_path = format!("themes/{}/style.css", config.theme);
    fs::copy(&style_path, output_dir.join("style.css"))?;

    println!("Site built successfully in 'public/' directory!");
    Ok(())
}
