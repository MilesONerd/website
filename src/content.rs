// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2026 Enzo (MilesONerd)

//! Parses Markdown files with YAML front matter into typed content.

use anyhow::{Context, Result};
use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{html, Options, Parser as MdParser};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Front matter shared by every simple content page (About, Contact, ...).
#[derive(Debug, Deserialize, Clone)]
pub struct PageFrontMatter {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Front matter for a blog post.
#[derive(Debug, Deserialize, Clone)]
pub struct PostFrontMatter {
    pub title: String,
    pub slug: String,
    pub date: String, // YYYY-MM-DD
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    /// If true, this post is excluded from production builds (Netlify sets
    /// the `NETLIFY` env var automatically) but still shows up locally.
    #[serde(default)]
    pub draft: bool,
}

/// A project entry for the Home page's `./projects` section.
#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Loads `content/projects.yaml` — a plain YAML list, no front matter/Markdown involved.
pub fn load_projects(path: &Path) -> Result<Vec<Project>> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    serde_yaml_ng::from_str(&raw).with_context(|| format!("invalid YAML in {}", path.display()))
}

pub struct Page {
    pub front: PageFrontMatter,
    pub html: String,
}

pub struct Post {
    pub front: PostFrontMatter,
    pub html: String,
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = MdParser::new_ext(markdown, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

/// Reads a single Markdown+front-matter file and splits it into front matter + rendered HTML.
fn read_matter<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<(T, String)> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;

    let matter = Matter::<YAML>::new();
    let parsed = matter
        .parse::<T>(&raw)
        .with_context(|| format!("invalid front matter in {}", path.display()))?;

    let front = parsed
        .data
        .with_context(|| format!("missing front matter in {}", path.display()))?;

    let html = markdown_to_html(&parsed.content);
    Ok((front, html))
}

pub fn load_page(path: &Path) -> Result<Page> {
    let (front, html) = read_matter::<PageFrontMatter>(path)?;
    Ok(Page { front, html })
}

pub fn load_post(path: &Path) -> Result<Post> {
    let (front, html) = read_matter::<PostFrontMatter>(path)?;
    Ok(Post { front, html })
}

/// Loads every `.md` file in a directory (non-recursive), sorted by filename.
pub fn load_dir<T>(dir: &Path, loader: impl Fn(&Path) -> Result<T>) -> Result<Vec<T>> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("reading dir {}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    entries.iter().map(|e| loader(&e.path())).collect()
}
