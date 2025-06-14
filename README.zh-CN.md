# RBlog

<p align="center">
  <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" width="150" alt="RBlog Logo">
</p>

<p align="center">
  <strong>一个用 Rust 编写的、快如闪电的、配置驱动的静态网站生成器。</strong>
</p>

[English](./README.md) | [简体中文](./README.zh-CN.md)

---

RBlog 是一个极简而强大的静态网站生成器。它接收一个存放 Markdown 文件的目录、一个主题和一个配置文件，并将它们转换为一个可随时部署的静态网站。其灵感来源于 Hugo 和 Zola 等工具的简洁性，但以 Rust 的高性能和安全性为基础构建。

## ✨ 功能特性

- **快如闪电**: 利用 Rust 的卓越性能，在毫秒之间构建网站。
- **配置驱动**: 一个 `config.toml` 文件搞定所有全局配置。
- **Markdown 内容**: 使用标准的 Markdown 语法和 TOML Front Matter 撰写文章。
- **简单的主题系统**: 通过修改 HTML 模板和单个 CSS 文件，轻松创建或定制主题。
- **约定优于配置**: 简单直观的目录结构，无需复杂设置。
- **零依赖部署**: 生成的 `public/` 目录可以托管在任何地方。

## 🚀 快速开始

### 环境要求

你需要安装 Rust 工具链。如果还未安装，请访问 [rustup.rs](https://rustup.rs/) 获取。

### 安装

目前，你可以通过克隆此仓库并从源码构建来安装 RBlog。

```bash
git clone https://github.com/hi-liyan/rblog.git
cd rblog
cargo install --path .
```
这会将 `rblog` 二进制文件安装到你的 Cargo bin 路径 (`~/.cargo/bin`)。

## 📖 使用方法

### 1. 创建你的博客目录结构

RBlog 期望一个特定的目录结构以便正常工作。

```
我的博客/
├── config.toml           # 你的博客的全局配置文件
|
├── content/                # 所有的 Markdown 文章都放在这里
│   └── first-post.md
|
└── themes/                 # 存放你的主题
    └── basic/              # 一个名为 "basic" 的主题
        ├── style.css       # 主题的主样式表
        └── templates/      # HTML 模板
            ├── base.html   # 基础布局
            ├── header.html # 网站头部片段
            ├── index.html  # 首页 / 文章列表页
            └── post.html   # 单篇文章页
```
你可以使用本项目中的目录结构作为起点。

### 2. 配置你的博客

编辑 `config.toml` 文件：

```toml
# config.toml
base_url = "https://example.com"
title = "我的神奇博客"
description = "一个分享想法的地方。"
theme = "basic" # 对应 `themes/` 目录下的文件夹名
```

### 3. 撰写你的第一篇文章

在 `content/` 目录下创建一个 `.md` 文件。使用由 `---` 分隔的 TOML Front Matter。

```markdown
<!-- content/first-post.md -->
---
title = "我的第一篇文章"
date = "2023-10-28"
description = "这篇文章的简短摘要。"
tags = ["rust", "入门"]
---

# 你好，世界！

这是我第一篇文章的内容，使用 Markdown 编写。
```

### 4. 构建你的网站

进入你博客的根目录 (`我的博客/`)，然后运行构建命令：

```bash
rblog build
```

静态网站将被生成在 `public/` 目录中。现在你可以将这个目录上传到任何静态托管服务，如 GitHub Pages、Netlify 或 Vercel。

## 🤝 参与贡献

我们欢迎任何形式的贡献！无论是提交 Bug 报告、功能请求还是 Pull Request，我们都非常感谢。

### 开发设置

1.  Fork 并克隆本仓库。
2.  为你的新功能或 Bug 修复创建一个新分支：`git checkout -b my-new-feature`。
3.  进行修改。
4.  运行构建命令以测试你的改动：`cargo run -- build`。
5.  提交你的改动并将它们推送到你的 Fork。
6.  开启一个 Pull Request。

### 可贡献的方向

这里有一些你可以参与贡献的想法：

- **新功能**:
    - `rblog serve`: 一个带有实时重新加载功能的本地开发服务器。
    - 生成站点地图 (`sitemap.xml`)。
    - 生成 RSS/Atom Feed。
    - 支持更多的模板引擎或 Front Matter 格式（例如 YAML）。
    - `rblog new <post-title>` 命令，用于快速创建新文章的脚手架。
- **主题**: 创建并分享新的主题。
- **文档**: 改进此 README 文件或添加更详细的指南。
- **Bug 修复**: 查找并修复现有代码库中的错误。

## 📄 许可证

本项目采用 MIT 许可证。详情请见 [LICENSE](LICENSE) 文件。