// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Result;
use crate::output::{CATALOG, SNAPSHOTS, project_path};
use roxmltree::{Document, Node};
use std::{collections::BTreeSet, fs};

const NS: &str = "http://www.iana.org/assignments";

#[derive(Debug)]
pub struct Registry {
    pub id: String,
    pub title: String,
    pub parent: Option<String>,
    pub xml: String,
    pub records: Vec<Vec<(String, String)>>,
}
#[derive(Debug)]
pub struct Group {
    pub id: String,
    pub title: String,
    pub updated: String,
    pub registries: Vec<Registry>,
}

pub fn validate_id(id: &str) -> Result<()> {
    if id.is_empty()
        || !id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err(format!("invalid registry group ID: {id:?}").into());
    }
    Ok(())
}

pub fn load_groups() -> Result<Vec<Group>> {
    let mut groups = Vec::new();
    let mut ids = BTreeSet::new();
    for entry in fs::read_dir(project_path(SNAPSHOTS))? {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "xml") {
            continue;
        }
        let xml = fs::read_to_string(&path)?;
        let group = parse_snapshot(&xml).map_err(|e| format!("{}: {e}", path.display()))?;
        if !ids.insert(group.id.clone()) {
            return Err(format!("duplicate group {}", group.id).into());
        }
        groups.push(group);
    }
    if groups.is_empty() {
        return Err("no XML snapshots found".into());
    }
    let catalog_path = project_path(CATALOG);
    if catalog_path.exists() {
        let catalog = fs::read_to_string(&catalog_path)?;
        validate_catalog(&catalog, &ids)?;
    }
    groups.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(groups)
}

pub fn validate_catalog(catalog: &str, ids: &BTreeSet<String>) -> Result<()> {
    let mut discovered = BTreeSet::new();
    for line in catalog
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let (alias, canonical) = line.split_once('\t').ok_or("invalid catalog entry")?;
        validate_id(alias)?;
        validate_id(canonical)?;
        if !discovered.insert(alias) {
            return Err(format!("duplicate catalog entry {alias}").into());
        }
        if !ids.contains(canonical) {
            return Err(format!("catalog group {alias} is missing snapshot {canonical}").into());
        }
    }
    if discovered.is_empty() {
        return Err("empty discovery catalog".into());
    }
    Ok(())
}

fn children<'a, 'b>(node: Node<'a, 'b>, name: &'static str) -> impl Iterator<Item = Node<'a, 'b>> {
    node.children().filter(move |n| n.has_tag_name((NS, name)))
}

fn text(node: Node<'_, '_>) -> String {
    node.descendants()
        .filter(|n| n.is_text())
        .filter_map(|n| n.text())
        .collect::<String>()
        .trim()
        .to_owned()
}

fn child_text(node: Node<'_, '_>, name: &'static str) -> String {
    children(node, name).next().map(text).unwrap_or_default()
}

pub fn parse_snapshot(xml: &str) -> Result<Group> {
    let doc = Document::parse(xml)?;
    let root = doc.root_element();
    if !root.has_tag_name((NS, "registry")) {
        return Err("expected IANA registry XML root".into());
    }
    let id = root.attribute("id").ok_or("missing group ID")?.to_owned();
    validate_id(&id)?;
    let mut registries = Vec::new();
    let mut ids = BTreeSet::new();
    for node in root
        .descendants()
        .filter(|n| n.has_tag_name((NS, "registry")))
    {
        let id = node
            .attribute("id")
            .ok_or("missing registry ID")?
            .to_owned();
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate registry ID {id}").into());
        }
        registries.push(Registry {
            id,
            title: child_text(node, "title"),
            parent: node
                .parent_element()
                .filter(|n| n.has_tag_name((NS, "registry")))
                .and_then(|n| n.attribute("id"))
                .map(str::to_owned),
            xml: xml[node.range()].to_owned(),
            records: children(node, "record")
                .map(|record| {
                    record
                        .children()
                        .filter(|n| n.is_element())
                        .map(|n| (n.tag_name().name().to_owned(), text(n)))
                        .collect()
                })
                .collect(),
        });
    }
    Ok(Group {
        id,
        title: child_text(root, "title"),
        updated: child_text(root, "updated"),
        registries,
    })
}

pub fn field<'a>(record: &'a [(String, String)], key: &str) -> Option<&'a str> {
    record
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(body: &str) -> String {
        format!(
            r#"<registry xmlns="http://www.iana.org/assignments" id="example-parameters"><title>Example</title>{body}</registry>"#
        )
    }

    #[test]
    fn invalid_xml_and_duplicate_ids_fail() {
        assert!(parse_snapshot("<html/>").is_err());
        assert!(parse_snapshot(&fixture("<registry id='x'/><registry id='x'/>")).is_err());
        assert!(parse_snapshot(&fixture("<registry/>")).is_err());
    }

    #[test]
    fn catalog_requires_every_discovered_canonical_snapshot() {
        let catalog = "# header\nmail-parameters\tsmtp\nsmtp\tsmtp\n";
        assert!(validate_catalog(catalog, &BTreeSet::from(["smtp".into()])).is_ok());
        assert!(validate_catalog(catalog, &BTreeSet::new()).is_err());
        assert!(validate_catalog("", &BTreeSet::new()).is_err());
        assert!(
            validate_catalog("smtp\tsmtp\nsmtp\tsmtp", &BTreeSet::from(["smtp".into()])).is_err()
        );
        assert!(validate_catalog("../bad\tsmtp", &BTreeSet::from(["smtp".into()])).is_err());
    }
}
