use serde::{Deserialize, Serialize};

// 对应 config.toml
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub base_url: String,
    pub title: String,
    pub description: String,
    pub theme: String,
}

// 对应 Markdown 文件的 Front Matter
#[derive(Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub description: String,
    pub tags: Vec<String>
}

// 内存中表示一篇文章的完整结构
#[derive(Serialize)]
pub struct Post {
    pub metadata: FrontMatter,
    pub content_html: String,
    pub permalink: String,
}