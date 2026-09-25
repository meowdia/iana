// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Result;
use crate::{
    cli::Selection,
    output::{CATALOG, SNAPSHOTS, project_path, write_if_changed},
    snapshot::{parse_snapshot, validate_id},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
};

pub fn run(selection: Selection) -> Result<()> {
    let all = matches!(selection, Selection::All);
    let groups = match selection {
        Selection::All => discover_online()?.into_iter().collect(),
        Selection::Groups(groups) => groups,
    };
    let catalog = fetch_all(&groups)?;
    if all {
        write_catalog(&catalog)?;
    }
    Ok(())
}

fn write_catalog(catalog: &BTreeMap<String, String>) -> Result<()> {
    let mut contents = String::from(
        "# SPDX-FileCopyrightText: 2026 Meowdia Community\n# SPDX-License-Identifier: MIT OR Apache-2.0\n# Discovered IANA group ID\tCanonical XML group ID\n",
    );
    for (discovered, canonical) in catalog {
        writeln!(contents, "{discovered}\t{canonical}").unwrap();
    }
    write_if_changed(&project_path(CATALOG), &contents)?;
    Ok(())
}

const FETCH_WORKERS: usize = 2;
const DOWNLOAD_ATTEMPTS: u32 = 6;
const DOWNLOAD_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);
const MAX_SNAPSHOT_BYTES: u64 = 64 * 1024 * 1024;

fn fetch_all(groups: &[String]) -> Result<BTreeMap<String, String>> {
    let batch_size = groups.len().div_ceil(FETCH_WORKERS).max(1);
    let batches = std::thread::scope(|scope| {
        let mut workers = Vec::new();
        for batch in groups.chunks(batch_size) {
            workers.push(scope.spawn(move || {
                batch
                    .iter()
                    .map(|id| fetch(id).map(|canonical| (id.clone(), canonical)))
                    .collect::<Vec<_>>()
            }));
        }
        workers
            .into_iter()
            .map(|worker| {
                worker
                    .join()
                    .map_err(|_| "snapshot download worker panicked".to_owned())
            })
            .collect::<Result<Vec<_>, _>>()
    })?;

    let mut catalog = BTreeMap::new();
    let mut failures = Vec::new();
    for result in batches.into_iter().flatten() {
        match result {
            Ok((id, canonical)) => {
                catalog.insert(id, canonical);
            }
            Err(error) => failures.push(error.to_string()),
        }
    }
    if failures.is_empty() {
        return Ok(catalog);
    }
    failures.sort();
    Err(format!(
        "{} snapshot downloads failed:\n{}",
        failures.len(),
        failures.join("\n")
    )
    .into())
}

fn download(url: &str) -> Result<String> {
    let mut error = None;
    for attempt in 0..DOWNLOAD_ATTEMPTS {
        match download_once(url) {
            Ok(body) => return Ok(body),
            Err(message) => error = Some(message),
        }
        if attempt + 1 < DOWNLOAD_ATTEMPTS {
            let delay = 2 << attempt;
            eprintln!("{}; retrying in {delay}s", error.as_ref().unwrap());
            std::thread::sleep(std::time::Duration::from_secs(delay));
        }
    }
    Err(error.unwrap())
}

fn download_once(url: &str) -> Result<String> {
    ureq::get(url)
        .config()
        .timeout_global(Some(DOWNLOAD_TIMEOUT))
        .build()
        .call()
        .map_err(|e| format!("fetch {url}: {e}"))?
        .body_mut()
        .with_config()
        .limit(MAX_SNAPSHOT_BYTES)
        .read_to_string()
        .map_err(|e| format!("read {url}: {e}").into())
}

pub fn discover_online() -> Result<BTreeSet<String>> {
    let groups = discover(&download("https://www.iana.org/protocols")?);
    if groups.is_empty() {
        return Err("IANA catalog contained no registry groups".into());
    }
    Ok(groups)
}

fn discover(html: &str) -> BTreeSet<String> {
    // Only assignment links on the official index; both quote styles occur in HTML.
    html.split("href=")
        .skip(1)
        .filter_map(|tail| {
            let quote = tail.chars().next()?;
            if quote != '\'' && quote != '"' {
                return None;
            }
            let href = tail[1..].split(quote).next()?;
            let path = href.strip_prefix("https://www.iana.org").unwrap_or(href);
            let id = path
                .strip_prefix("/assignments/")?
                .split(['/', '#', '?'])
                .next()?;
            validate_id(id).ok()?;
            Some(id.to_owned())
        })
        .collect()
}

fn fetch(id: &str) -> Result<String> {
    let url = format!("https://www.iana.org/assignments/{id}/{id}.xml");
    let xml = download(&url)?;
    let group = parse_snapshot(&xml)?;
    // Redirects in IANA's catalog can name aliases of the same XML document.
    let path = project_path(SNAPSHOTS).join(format!("{}.xml", group.id));
    fs::create_dir_all(project_path(SNAPSHOTS))?;
    let temporary = project_path(SNAPSHOTS).join(format!(".{id}.tmp"));
    fs::write(&temporary, &xml)?;
    fs::rename(&temporary, &path)?;
    println!("fetched {id} -> {}", group.id);
    Ok(group.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_deduplicates_and_rejects_external_or_unsafe_links() {
        let groups = discover(
            r#"<a href="/assignments/tls-parameters/#one"></a>
            <a href='https://www.iana.org/assignments/sdp-parameters/sdp-parameters.xhtml'></a>
            <a href="/assignments/tls-parameters/#two"></a>
            <a href="https://evil.test/assignments/evil/"></a>
            <a href="/assignments/../secret"></a>"#,
        );
        assert_eq!(
            groups.into_iter().collect::<Vec<_>>(),
            ["sdp-parameters", "tls-parameters"]
        );
        assert!(validate_id("../secret").is_err());
    }
}
