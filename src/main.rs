// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2026 Enzo (MilesONerd)

//! `gen-content`: reads Markdown+YAML content and renders the whole site
//! to static HTML (via Leptos SSR) plus an Atom feed, sitemap.xml and
//! robots.txt, ready for Netlify.

mod components;
mod config;
mod content;
mod feed;
mod pages;
mod sitemap;

use anyhow::{Context, Result};
use sitemap::StaticRoute;
use std::fs;
use std::path::{Path, PathBuf};

const CONTENT_DIR: &str = "content";
const OUT_DIR: &str = "dist";

const STATIC_ROUTES: &[StaticRoute] = &[
    StaticRoute { path: "/", priority: "1.0" },
    StaticRoute { path: "/about/", priority: "0.8" },
    StaticRoute { path: "/blog/", priority: "0.8" },
    StaticRoute { path: "/contact/", priority: "0.5" },
    StaticRoute { path: "/privacy-policy/", priority: "0.3" },
    StaticRoute { path: "/terms-of-service/", priority: "0.3" },
];

fn main() -> Result<()> {
    let root = PathBuf::from(CONTENT_DIR);
    let out = PathBuf::from(OUT_DIR);

    if out.exists() {
        fs::remove_dir_all(&out).context("clearing dist/")?;
    }
    fs::create_dir_all(&out)?;

    // --- load content -----------------------------------------------------
    let pages_dir = root.join("pages");
    let blog_dir = root.join("blog");

    let about = content::load_page(&pages_dir.join("about.md"))?;
    let contact = content::load_page(&pages_dir.join("contact.md"))?;
    let privacy = content::load_page(&pages_dir.join("privacy.md"))?;
    let terms = content::load_page(&pages_dir.join("terms.md"))?;
    let not_found = content::load_page(&pages_dir.join("404.md"))?;
    let projects = content::load_projects(&root.join("projects.yaml"))?;

    let mut posts = content::load_dir(&blog_dir, content::load_post)?;
    // newest first
    posts.sort_by(|a, b| b.front.date.cmp(&a.front.date));

    // Netlify sets the `NETLIFY` env var automatically on their build
    // servers — if it's absent, we're building locally, so drafts stay in.
    let is_production = std::env::var_os("NETLIFY").is_some();
    if is_production {
        let before = posts.len();
        posts.retain(|p| !p.front.draft);
        let dropped = before - posts.len();
        if dropped > 0 {
            println!("gen-content: production build — hiding {dropped} draft post(s)");
        }
    }

    // --- render pages -------------------------------------------------------
    write_page(&out.join("index.html"), pages::render_home(&posts[..posts.len().min(5)], &projects))?;
    write_page(&out.join("about/index.html"), pages::render_static_page("/about/", "about", &about))?;
    write_page(&out.join("contact/index.html"), pages::render_contact_page(&contact))?;
    write_page(&out.join("privacy-policy/index.html"), pages::render_static_page("/privacy-policy/", "privacy-policy", &privacy))?;
    write_page(&out.join("terms-of-service/index.html"), pages::render_static_page("/terms-of-service/", "terms-of-service", &terms))?;
    write_page(&out.join("blog/index.html"), pages::render_blog_index(&posts))?;
    write_page(&out.join("404.html"), pages::render_static_page("/404/", "404", &not_found))?;

    for post in &posts {
        let path = out.join(format!("blog/{}/index.html", post.front.slug));
        write_page(&path, pages::render_post(post))?;
    }

    // --- atom feed, sitemap, robots.txt ----------------------------------------
    let atom = feed::build_atom_feed(&posts)?;
    fs::write(out.join("atom.xml"), atom).context("writing atom.xml")?;

    let build_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let sitemap_xml = sitemap::build_sitemap(STATIC_ROUTES, &posts, &build_date);
    fs::write(out.join("sitemap.xml"), sitemap_xml).context("writing sitemap.xml")?;

    fs::write(out.join("robots.txt"), sitemap::build_robots_txt()).context("writing robots.txt")?;

    // --- copy static assets (favicon, etc.) -------------------------------------
    // NOTE: style.css is written directly into dist/ by the Tailwind CLI step
    // that runs *after* this binary in the build command (see netlify.toml),
    // since this binary clears dist/ on every run.
    copy_if_exists(Path::new("static"), &out)?;

    println!(
        "gen-content: wrote {} pages + {} posts + atom.xml + sitemap.xml + robots.txt to {}/",
        STATIC_ROUTES.len() + 1,
        posts.len(),
        OUT_DIR
    );
    Ok(())
}

fn write_page(path: &Path, html: String) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, html).with_context(|| format!("writing {}", path.display()))
}

fn copy_if_exists(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() {
        return Ok(());
    }
    if src.is_dir() {
        for entry in walkdir::WalkDir::new(src).min_depth(1) {
            let entry = entry?;
            let rel = entry.path().strip_prefix(src)?;
            let target = dst.join(rel);
            if entry.file_type().is_dir() {
                fs::create_dir_all(&target)?;
            } else {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(entry.path(), &target)?;
            }
        }
    } else {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dst)?;
    }
    Ok(())
}
