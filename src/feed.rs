// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2026 Enzo (MilesONerd)

use crate::config::SITE_URL;
use crate::content::Post;
use anyhow::{Context, Result};
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone, Utc};

fn parse_date(date: &str) -> Result<DateTime<FixedOffset>> {
    let naive = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .with_context(|| format!("invalid date '{date}', expected YYYY-MM-DD"))?;
    let naive_dt = naive
        .and_hms_opt(12, 0, 0)
        .context("invalid time component")?;
    let utc: DateTime<Utc> = Utc.from_utc_datetime(&naive_dt);
    Ok(utc.into())
}

pub fn build_atom_feed(posts: &[Post]) -> Result<String> {
    let author = PersonBuilder::default().name("MilesONerd".to_string()).build();

    let mut entries: Vec<Entry> = Vec::with_capacity(posts.len());
    for post in posts {
        let url = format!("{SITE_URL}/blog/{}/", post.front.slug);
        let updated = parse_date(&post.front.date)?;

        let entry = EntryBuilder::default()
            .title(post.front.title.clone())
            .id(url.clone())
            .updated(updated)
            .authors(vec![author.clone()])
            .links(vec![LinkBuilder::default().href(url).build()])
            .summary(post.front.summary.clone().map(Into::into))
            .content(
                Some(
                    ContentBuilder::default()
                        .content_type(Some("html".to_string()))
                        .value(Some(post.html.clone()))
                        .build(),
                ),
            )
            .build();

        entries.push(entry);
    }

    let latest_update = entries
        .iter()
        .map(|e| e.updated)
        .max()
        .unwrap_or_else(|| Utc::now().into());

    let feed = FeedBuilder::default()
        .title("MilesONerd — Blog".to_string())
        .id(format!("{SITE_URL}/"))
        .updated(latest_update)
        .authors(vec![author])
        .links(vec![
            LinkBuilder::default()
                .href(format!("{SITE_URL}/atom.xml"))
                .rel("self")
                .build(),
            LinkBuilder::default().href(format!("{SITE_URL}/")).build(),
        ])
        .rights(Some("© Enzo Costa Fuke. Content licensed CC BY 4.0.".to_string().into()))
        .entries(entries)
        .build();

    Ok(feed.to_string())
}
