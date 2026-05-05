use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::models::{Link, PagedResponse};

fn op_read(reference: &str) -> Result<String> {
    let output = Command::new("op")
        .args(["read", reference])
        .output()
        .context("Failed to run 'op' (1Password CLI). Is it installed and signed in?")?;
    if !output.status.success() {
        bail!(
            "op read failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8(output.stdout)
        .context("op read returned non-UTF-8 output")?
        .trim()
        .to_string())
}

fn fetch_highlighted_links(base_url: &str, token: &str) -> Result<Vec<Link>> {
    let agent = ureq::AgentBuilder::new().build();
    let mut links = Vec::new();
    let mut offset = 0usize;
    let limit = 100;
    loop {
        let url = format!(
            "{}/api/v1/lists/highlighted?limit={}&offset={}&includeRead=true",
            base_url, limit, offset
        );
        let resp: PagedResponse<Link> = agent
            .get(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .call()
            .context("Failed to fetch highlighted links from GoodLinks")?
            .into_json()
            .context("Failed to parse highlighted links response")?;
        let has_more = resp.has_more;
        let count = resp.data.len();
        links.extend(resp.data);
        if !has_more || count == 0 {
            break;
        }
        offset += count;
    }
    Ok(links)
}

/// Returns `None` if the link has no highlights (404 response).
fn fetch_highlights_export(
    agent: &ureq::Agent,
    base_url: &str,
    token: &str,
    link_id: &str,
) -> Result<Option<String>> {
    let url = format!("{}/api/v1/links/{}/highlights/export", base_url, link_id);
    match agent
        .get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
    {
        Ok(resp) => {
            let markdown = resp
                .into_string()
                .context("Failed to read highlights export response")?;
            Ok(Some(markdown))
        }
        Err(ureq::Error::Status(404, _)) => Ok(None),
        Err(e) => Err(e).context("Failed to fetch highlights export from GoodLinks"),
    }
}

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn write_highlight_file(source_dir: &Path, link: &Link, markdown: &str) -> Result<PathBuf> {
    let title = link.title.as_deref().unwrap_or(&link.id);
    let filename = format!("{}.md", sanitize_filename(title));
    let file_path = source_dir.join(&filename);
    fs::write(&file_path, markdown)
        .with_context(|| format!("Failed to write {}", file_path.display()))?;
    Ok(file_path)
}

pub fn import(source: &Path, dry_run: bool, verbose: bool) -> Result<()> {
    let base_url = op_read("op://Private/GoodLinks/base_url")
        .context("Failed to read GoodLinks endpoint from 1Password")?;
    let token = op_read("op://Private/GoodLinks/token")
        .context("Failed to read GoodLinks token from 1Password")?;

    fs::create_dir_all(source)
        .with_context(|| format!("Failed to create source directory {}", source.display()))?;

    let links = fetch_highlighted_links(&base_url, &token)?;
    if verbose {
        println!("Found {} highlighted links", links.len());
    }

    let agent = ureq::AgentBuilder::new().build();
    for link in &links {
        let Some(markdown) = fetch_highlights_export(&agent, &base_url, &token, &link.id)? else {
            continue;
        };
        if verbose {
            let title = link.title.as_deref().unwrap_or(&link.id);
            if dry_run {
                println!("Would fetch highlights for: {}", title);
            } else {
                println!("Fetched highlights for: {}", title);
            }
        }
        if !dry_run {
            write_highlight_file(source, link, &markdown)?;
        }
    }

    Ok(())
}
