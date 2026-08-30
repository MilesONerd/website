// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2026 Enzo (MilesONerd)

//! Reusable Leptos components shared across every generated page.
//! These render to plain HTML strings at build time (SSR) — no WASM
//! ships to the browser.

use crate::config::SITE_NAME;
use crate::content::Project;
use leptos::prelude::*;

pub struct NavLink {
    pub href: &'static str,
    pub label: &'static str,
}

/// Top nav entries, shown in terminal "./slug" style.
pub const NAV: &[NavLink] = &[
    NavLink { href: "/about/", label: "./about" },
    NavLink { href: "/blog/", label: "./blog" },
    NavLink { href: "/contact/", label: "./contact" },
];

/// Site-wide chrome: `<head>`, header nav, footer, and license notices.
/// `active` is the current nav href, used to highlight the link.
#[component]
pub fn Layout(
    title: String,
    description: String,
    active: &'static str,
    children: Children,
) -> impl IntoView {
    let year = chrono::Utc::now().format("%Y").to_string();
    let full_title = format!("{title} — {SITE_NAME}");

    view! {
        <!DOCTYPE html>
        <html lang="en" class="scroll-smooth">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{full_title}</title>
                <meta name="description" content=description />
                <link rel="alternate" type="application/atom+xml" title="Blog feed" href="/atom.xml" />
                <link rel="stylesheet" href="/style.css" />
                <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
                <link rel="icon" href="/favicon.ico" type="image/x-icon" />

                <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
                <link rel="icon" type="image/png" sizes="192x192" href="/android-chrome-192x192.png" />
                <link rel="icon" type="image/png" sizes="512x512" href="/android-chrome-512x512.png" />
                <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
            </head>
            <body class="min-h-screen bg-black text-white font-mono antialiased flex flex-col">
                <SiteHeader active=active />
                <main class="flex-1 w-full max-w-3xl mx-auto px-6 py-12">
                    {children()}
                </main>
                <SiteFooter year=year />
            </body>
        </html>
    }
}

/// The "~/milesonerd" home link with a blinking terminal-cursor before the `~`.
#[component]
pub fn SiteHeader(active: &'static str) -> impl IntoView {
    let home_label = format!("~/{SITE_NAME}");
    view! {
        <header class="border-b border-neutral-800">
            <nav class="max-w-3xl mx-auto px-6 py-5 flex items-center justify-between flex-wrap gap-4">
                <a href="/" class="flex items-center text-sm tracking-tight text-white hover:text-red-500 transition-colors">
                    <span class="cursor-blink text-red-500" aria-hidden="true">"_"</span>
                    <span>{home_label}</span>
                </a>
                <ul class="flex gap-6 text-sm">
                    {NAV.iter().map(|link| {
                        let is_active = link.href == active;
                        let classes = if is_active {
                            "text-red-500"
                        } else {
                            "text-neutral-400 hover:text-red-500 transition-colors"
                        };
                        view! {
                            <li>
                                <a href=link.href class=classes>{link.label}</a>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </nav>
        </header>
    }
}

#[component]
pub fn SiteFooter(year: String) -> impl IntoView {
    let copyright = format!("© {year} Enzo Costa Fuke. Code licensed ");
    view! {
        <footer class="border-t border-neutral-800 text-xs text-neutral-500">
            <div class="max-w-3xl mx-auto px-6 py-8 flex flex-col gap-2">
                <p>
                    {copyright}
                    <a class="text-red-500 hover:text-red-400 underline" href="https://github.com/MilesONerd/website" target="_blank" rel="noopener">
                        "Apache-2.0 OR MIT"
                    </a>
                    "."
                </p>
                <p>
                    "Content (posts and pages) licensed "
                    <a class="text-red-500 hover:text-red-400 underline" href="https://creativecommons.org/licenses/by/4.0/" target="_blank" rel="noopener">
                        "CC BY 4.0"
                    </a>
                    "."
                </p>
                <p>
                   "Built with Rust 🦀 · Powered by Open Source 💙"
                </p>
                <p class="flex gap-4">
                    <a class="hover:text-red-500 transition-colors" href="/privacy-policy/">"./privacy-policy"</a>
                    <a class="hover:text-red-500 transition-colors" href="/terms-of-service/">"./terms-of-service"</a>
                    <a class="hover:text-red-500 transition-colors" href="/sitemap.xml">"./sitemap.xml"</a>
                    <a class="hover:text-red-500 transition-colors" href="/atom.xml">"./atom.xml"</a>
                </p>
            </div>
        </footer>
    }
}

/// A route heading rendered in terminal filename style, e.g. `./privacy-policy`.
#[component]
pub fn RouteHeading(route: String) -> impl IntoView {
    view! {
        <h1 class="text-2xl text-white mb-6">
            <span class="text-red-500">"./"</span>
            {route}
        </h1>
    }
}

/// A single post entry in the blog index list.
#[component]
pub fn PostCard(title: String, href: String, date: String, summary: String, draft: bool) -> impl IntoView {
    view! {
        <li class="border-b border-neutral-800 py-6 first:pt-0 last:border-0">
            <a href=href class="group block">
                <div class="flex items-center gap-2">
                    <time class="text-xs text-neutral-500">{date}</time>
                    {draft.then(|| view! {
                        <span class="text-xs text-red-500 border border-red-500 px-1 rounded">"DRAFT"</span>
                    })}
                </div>
                <h2 class="mt-1 text-lg text-white group-hover:text-red-500 transition-colors">
                    <span class="text-red-500 group-hover:text-white transition-colors">"> "</span>
                    {title}
                </h2>
                <p class="mt-1 text-sm text-neutral-400">{summary}</p>
            </a>
        </li>
    }
}

/// A single entry in the `./projects` section on Home.
#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let name = project.name;
    let description = project.description;
    let tags = project.tags;
    let url = project.url;

    let heading = view! {
        <h3 class="text-white group-hover:text-red-500 transition-colors">
            <span class="text-red-500 group-hover:text-white transition-colors">"$ "</span>
            {name}
        </h3>
    };

    view! {
        <li class="border border-neutral-800 rounded p-4 hover:border-red-500 transition-colors">
            {match url {
                Some(href) => view! {
                    <a href=href target="_blank" rel="noopener" class="group block">
                        {heading}
                        <p class="mt-1 text-sm text-neutral-400">{description.clone()}</p>
                    </a>
                }.into_any(),
                None => view! {
                    <div class="group">
                        {heading}
                        <p class="mt-1 text-sm text-neutral-400">{description.clone()}</p>
                    </div>
                }.into_any(),
            }}
            <div class="mt-3 flex gap-2 flex-wrap">
                {tags.into_iter().map(|t| view! {
                    <span class="text-xs px-2 py-0.5 rounded border border-neutral-800 text-neutral-500">
                        {t}
                    </span>
                }).collect_view()}
            </div>
        </li>
    }
}

/// Contact form that POSTs directly to Formspree — no JavaScript needed.
#[component]
pub fn ContactForm() -> impl IntoView {
    let action = format!("https://formspree.io/f/{}", crate::config::FORMSPREE_FORM_ID);
    view! {
        <form action=action method="POST" class="mt-8 flex flex-col gap-4 max-w-md">
            <label class="flex flex-col gap-1 text-sm text-neutral-400">
                <span><span class="text-red-500">"./"</span>"name"</span>
                <input
                    type="text"
                    name="name"
                    required
                    class="bg-black border border-neutral-800 text-white px-3 py-2 rounded focus:outline-none focus:border-red-500"
                />
            </label>
            <label class="flex flex-col gap-1 text-sm text-neutral-400">
                <span><span class="text-red-500">"./"</span>"email"</span>
                <input
                    type="email"
                    name="email"
                    required
                    class="bg-black border border-neutral-800 text-white px-3 py-2 rounded focus:outline-none focus:border-red-500"
                />
            </label>
            <label class="flex flex-col gap-1 text-sm text-neutral-400">
                <span><span class="text-red-500">"./"</span>"message"</span>
                <textarea
                    name="message"
                    required
                    rows="5"
                    class="bg-black border border-neutral-800 text-white px-3 py-2 rounded focus:outline-none focus:border-red-500"
                ></textarea>
            </label>
            // Formspree option: redirect back here with a query param instead of their default "thanks" page.
            <input type="hidden" name="_next" value="/contact/?sent=1" />
            <button
                type="submit"
                class="mt-2 border border-red-500 text-red-500 px-4 py-2 rounded hover:bg-red-500 hover:text-black transition-colors self-start"
            >
                "$ send"
            </button>
        </form>
    }
}

/// Renders raw HTML (already-converted Markdown) inside a typographic wrapper.
#[component]
pub fn Prose(html: String) -> impl IntoView {
    view! {
        <div class="prose-content" inner_html=html></div>
    }
}
