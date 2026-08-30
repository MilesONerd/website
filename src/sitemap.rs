// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2026 Enzo (MilesONerd)

use crate::config::SITE_URL;
use crate::content::Post;

/// A static route to include in the sitemap, with an XML-escaped last-modified date.
pub struct StaticRoute {
    pub path: &'static str,
    pub priority: &'static str,
}

pub fn build_sitemap(static_routes: &[StaticRoute], posts: &[Post], build_date: &str) -> String {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    for route in static_routes {
        xml.push_str(&format!(
            "<url><loc>{SITE_URL}{}</loc><lastmod>{build_date}</lastmod><priority>{}</priority></url>",
            route.path, route.priority
        ));
    }

    for post in posts {
        xml.push_str(&format!(
            "<url><loc>{SITE_URL}/blog/{}/</loc><lastmod>{}</lastmod><priority>0.6</priority></url>",
            post.front.slug, post.front.date
        ));
    }

    xml.push_str("</urlset>");
    xml
}

pub fn build_robots_txt() -> String {
    format!("User-agent: *\nAllow: /\nSitemap: {SITE_URL}/sitemap.xml\n")
}
