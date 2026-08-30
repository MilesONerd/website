// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2026 Enzo (MilesONerd)

use crate::components::{Layout, PostCard, ProjectCard, Prose, RouteHeading, ContactForm};
use crate::content::{Page, Post, Project};
use leptos::prelude::*;

pub fn render_home(featured: &[Post], projects: &[Project]) -> String {
    let posts = featured.to_vec_owned();
    let projects = projects.to_vec();
    let view = view! {
        <Layout
            title="Home".to_string()
            description="MilesONerd — system developer.".to_string()
            active="/"
        >
            <section>
                <p class="text-neutral-500">"$ whoami"</p>
                <h1 class="text-2xl text-white mt-1">"milesonerd"</h1>
                <p class="mt-3 text-neutral-400 max-w-prose">
                    "Self-taught developer and systems programmer, focused on low-level "
                    "systems and developer tooling. Who loves Rust, C, and Zig."
                </p>
            </section>

            <section class="mt-12">
                <h2 class="text-sm text-neutral-500 mb-4">
                    <span class="text-red-500">"./"</span>
                    "projects"
                </h2>
                <ul class="grid gap-4 sm:grid-cols-2">
                    {projects.into_iter().map(|p| view! { <ProjectCard project=p /> }).collect_view()}
                </ul>
            </section>

            <section class="mt-12">
                <h2 class="text-sm text-neutral-500 mb-4">
                    <span class="text-red-500">"./"</span>
                    "blog"
                </h2>
                <ul>
                    {posts.into_iter().map(|p| {
                        view! {
                            <PostCard
                                title=p.front.title
                                href=format!("/blog/{}/", p.front.slug)
                                date=p.front.date
                                summary=p.front.summary.unwrap_or_default()
                                draft=p.front.draft
                            />
                        }
                    }).collect_view()}
                </ul>
            </section>
        </Layout>
    };
    render_document(view)
}

/// Renders a static content page (About, Contact, Privacy, Terms, 404).
/// `route` is the terminal-style route label, e.g. `"about"` -> shown as `./about`.
pub fn render_static_page(active: &'static str, route: &str, page: &Page) -> String {
    let title = page.front.title.clone();
    let description = page.front.description.clone().unwrap_or_default();
    let html = page.html.clone();
    let route = route.to_string();
    let view = view! {
        <Layout title=title description=description active=active>
            <RouteHeading route=route />
            <Prose html=html />
        </Layout>
    };
    render_document(view)
}

/// Contact page: same as a static page, plus the Formspree contact form.
pub fn render_contact_page(page: &Page) -> String {
    let title = page.front.title.clone();
    let description = page.front.description.clone().unwrap_or_default();
    let html = page.html.clone();
    let view = view! {
        <Layout title=title description=description active="/contact/">
            <RouteHeading route="contact".to_string() />
            <Prose html=html />
            <ContactForm />
        </Layout>
    };
    render_document(view)
}

pub fn render_blog_index(posts: &[Post]) -> String {
    let posts = posts.to_vec_owned();
    let view = view! {
        <Layout
            title="Blog".to_string()
            description="Posts about systems, tooling and other technologies things.".to_string()
            active="/blog/"
        >
            <RouteHeading route="blog".to_string() />
            <ul>
                {posts.into_iter().map(|p| view! {
                    <PostCard
                        title=p.front.title
                        href=format!("/blog/{}/", p.front.slug)
                        date=p.front.date
                        summary=p.front.summary.unwrap_or_default()
                        draft=p.front.draft
                    />
                }).collect_view()}
            </ul>
        </Layout>
    };
    render_document(view)
}

pub fn render_post(post: &Post) -> String {
    let title = post.front.title.clone();
    let summary = post.front.summary.clone().unwrap_or_default();
    let date = post.front.date.clone();
    let html = post.html.clone();
    let tags = post.front.tags.clone();
    let breadcrumb = format!("blog/{}", post.front.slug);
    let draft = post.front.draft;
    let view = view! {
        <Layout title=title.clone() description=summary active="/blog/">
            <article>
                <RouteHeading route=breadcrumb />
                <div class="flex items-center gap-2">
                    <time class="text-xs text-neutral-500">{date}</time>
                    {draft.then(|| view! {
                        <span class="text-xs text-red-500 border border-red-500 px-1 rounded">"DRAFT"</span>
                    })}
                </div>
                <h2 class="text-xl text-white mt-1 mb-6">{title}</h2>
                <Prose html=html />
                <div class="mt-8 flex gap-2 flex-wrap">
                    {tags.into_iter().map(|t| view! {
                        <span class="text-xs px-2 py-1 rounded border border-neutral-800 text-neutral-500">
                            {t}
                        </span>
                    }).collect_view()}
                </div>
            </article>
        </Layout>
    };
    render_document(view)
}

fn render_document(view: impl IntoView) -> String {
    format!("<!DOCTYPE html>\n{}", view.to_html())
}

// Small helper: `Post` doesn't derive Clone in content.rs, so this documents
// the pattern used to pass owned copies into views built from a `&[Post]`.
trait ToVecOwned<T> {
    fn to_vec_owned(&self) -> Vec<T>;
}

impl ToVecOwned<Post> for [Post] {
    fn to_vec_owned(&self) -> Vec<Post> {
        self.iter()
            .map(|p| Post {
                front: p.front.clone(),
                html: p.html.clone(),
            })
            .collect()
    }
}
