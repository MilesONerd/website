# milesonerd — Portfolio & Blog

<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/MilesONerd/yt-tui)

Personal website (portfolio + blog) with a terminal aesthetic (black, white,
red accents). Generated as 100% static HTML at build time by a Rust binary
that uses [Leptos](https://leptos.dev) components in SSR mode (no WASM sent
to the browser) and Markdown content with YAML front matter. Published on
Netlify.

## How it works

```
content/
  pages/            # About, Contact, Privacy Policy, Terms of Service, 404
  blog/              # Posts (Markdown + YAML front matter), one file per post
  projects.yaml       # List of projects displayed in the ./projects section of the home page
src/
  content.rs         # Front matter parser + Markdown -> HTML, + projects.yaml loader
  components.rs       # Reusable Leptos components (Layout, Header, Footer, PostCard, ProjectCard...)
  pages.rs            # Page composition from components
  feed.rs             # Atom feed generation (atom_syndication)
  sitemap.rs           # sitemap.xml and robots.txt generation
  config.rs            # Shared constants (SITE_URL, SITE_NAME)
  main.rs              # Orchestrator: reads content/, renders everything, writes dist/
style/input.css        # Tailwind directives + blinking cursor animation
dist/                    # Final output (generated at build, not versioned)
```

`cargo run --release --bin gen-content` reads everything in `content/`,
renders each page with Leptos components (`View::to_html()`), writes the HTML
to `dist/`, and generates `dist/atom.xml`, `dist/sitemap.xml`, and
`dist/robots.txt` from the same data. Then, the Tailwind CLI scans
`src/**/*.rs` for classes used inside `view!{}` macros and writes
`dist/style.css`.

See `netlify.toml` for the exact build command and `rust-toolchain.toml`
for the Rust version (`stable`, automatically resolved by `rustup` already
present in Netlify's build image).

## Running locally

```sh
cargo run --release --bin gen-content
bun install
bun run build:css
bunx serve dist
```

## Adding a post

Create `content/blog/AAAA-MM-DD-slug.md`:

```yaml
---
title: "Post title"
slug: "post-slug"
date: "2026-08-29"
summary: "Short summary (shown in the listing and the Atom feed)."
tags: ["rust"]
---
Regular Markdown content here.
```

No Rust code needs to be touched — the next build will pick up the post,
generate `dist/blog/post-slug/index.html`, and include the entry in
`dist/atom.xml` and `dist/sitemap.xml`.

### Drafts (`draft: true`)

Add `draft: true` to a post's front matter to mark it as a draft. Locally
(`cargo run`) it appears normally, with a red `DRAFT` badge. In production
builds, `gen-content` detects the `NETLIFY` environment variable (which
Netlify automatically sets in their builds) and removes the post from `dist/`,
the Atom feed, and the `sitemap.xml` — without needing to change anything
manually.

## Adding a project

Edit `content/projects.yaml` and add an entry:

```yaml
- name: "Project name"
  description: "Short description."
  url: "https://github.com/MilesONerd/repo"
  tags: ["rust"]
```

`url` is optional — without it, the project card won't be a clickable link.

## Licenses

- **Code** (`src/`, build configuration): dual licensed under
  [Apache-2.0](./LICENSE-APACHE) OR [MIT](./LICENSE-MIT), your choice.
  Each `.rs` file/config carries the header
  `SPDX-License-Identifier: Apache-2.0 OR MIT`.
- **Content** (posts and pages in `content/`): [CC BY 4.0](./LICENSE-CONTENT.md).
