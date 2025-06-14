# RBlog

<p align="center">
  <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" width="150" alt="RBlog Logo">
</p>

<p align="center">
  <strong>A blazing fast, configuration-driven static site generator written in Rust.</strong>
</p>

[English](./README.md) | [简体中文](./README.zh-CN.md)

---

RBlog is a minimalist yet powerful static site generator. It takes a directory of Markdown files, a theme, and a configuration file, and transforms them into a ready-to-deploy static website. Inspired by the simplicity of tools like Hugo and Zola, but built with the performance and safety of Rust.

## ✨ Features

- **Blazing Fast**: Leverages Rust's performance to build sites in milliseconds.
- **Configuration Driven**: A single `config.toml` file to rule them all.
- **Markdown Content**: Write your posts in standard Markdown with TOML front matter.
- **Simple Theming**: Easily create or customize themes by modifying HTML templates and a single CSS file.
- **Convention over Configuration**: A simple and intuitive directory structure. No complex setup needed.
- **Zero-Dependency Deployment**: The output `public/` directory can be hosted anywhere.

## 🚀 Getting Started

### Prerequisites

You need to have the Rust toolchain installed. If you don't have it, get it from [rustup.rs](https://rustup.rs/).

### Installation

Currently, you can install RBlog by cloning this repository and building it from source.

```bash
git clone https://github.com/hi-liyan/rblog.git
cd rblog
cargo install --path .
```
This will install the `rblog` binary to your Cargo bin path (`~/.cargo/bin`).

## 📖 Usage

### 1. Create Your Blog's Directory Structure

RBlog expects a specific directory structure to work correctly.

```
my-awesome-blog/
├── config.toml           # Your blog's global configuration
|
├── content/                # All your Markdown posts go here
│   └── first-post.md
|
└── themes/                 # Houses your theme(s)
    └── basic/              # A theme named "basic"
        ├── style.css       # The main stylesheet for the theme
        └── templates/      # HTML templates
            ├── base.html   # Base layout
            ├── header.html # Site header partial
            ├── index.html  # Homepage / Post list
            └── post.html   # Single post page
```
You can use the structure within this repository as a starting point.

### 2. Configure Your Blog

Edit the `config.toml` file:

```toml
# config.toml
base_url = "https://example.com"
title = "My Awesome Blog"
description = "A place to share my thoughts."
theme = "basic" # Corresponds to the folder name in `themes/`
```

### 3. Write Your First Post

Create a `.md` file inside the `content/` directory. Use YAML front matter delimited by `---`.

```markdown
<!-- content/first-post.md -->
---
title = "My First Post"
date = "2023-10-28"
description = "A short summary of this post."
tags = ["rust", "intro"]
---

# Hello, World!

This is the content of my first post, written in Markdown.
```

### 4. Build Your Site

Navigate to the root of your blog directory (`my-awesome-blog/`) and run the build command:

```bash
rblog build
```

The static site will be generated in the `public/` directory. You can now upload this directory to any static hosting service like GitHub Pages, Netlify, or Vercel.

## 🤝 Contributing

Contributions are welcome! Whether it's bug reports, feature requests, or pull requests, we appreciate all help.

### Development Setup

1.  Fork and clone the repository.
2.  Create a new branch for your feature or bug fix: `git checkout -b my-new-feature`.
3.  Make your changes.
4.  Run the build command to test your changes: `cargo run -- build`.
5.  Commit your changes and push them to your fork.
6.  Open a Pull Request.

### Areas for Contribution

Here are some ideas for how you can contribute:

- **New Features**:
    - `rblog serve`: A local development server with live-reloading.
    - Sitemap generation (`sitemap.xml`).
    - RSS/Atom feed generation.
    - Support for more template engines or front matter formats (e.g., YAML).
    - A `rblog new <post-title>` command to scaffold new posts.
- **Theming**: Create and share new themes.
- **Documentation**: Improve this README or add more detailed guides.
- **Bug Fixes**: Find and fix bugs in the existing codebase.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.