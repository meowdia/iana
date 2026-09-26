// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    Result,
    output::{GENERATED, format_rust_source, project_path, write_if_changed},
    snapshot::{Group, Registry, field, load_groups},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

pub enum Mode {
    Write,
    Check,
}

fn ident(value: &str, upper: bool) -> String {
    let words: Vec<_> = value
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();
    let mut result = if upper {
        words
            .iter()
            .map(|word| {
                format!(
                    "{}{}",
                    word[..1].to_ascii_uppercase(),
                    word[1..].to_ascii_lowercase()
                )
            })
            .collect()
    } else {
        words.join("_").to_ascii_lowercase()
    };
    if result.is_empty() {
        result.push_str("value");
    }
    if result.starts_with(|c: char| c.is_ascii_digit()) {
        result.insert(0, if upper { 'V' } else { 'v' });
    }
    if syn::parse_str::<syn::Ident>(&result).is_err() {
        result.push('_');
    }
    result
}

fn unique(base: String, used: &mut BTreeSet<String>) -> String {
    if used.insert(base.clone()) {
        return base;
    }
    for i in 2.. {
        let name = format!("{base}{i}");
        if used.insert(name.clone()) {
            return name;
        }
    }
    unreachable!()
}

fn number(value: &str) -> Option<u64> {
    let value = value.trim();
    if value.contains(',') {
        return value.split(',').try_fold(0u64, |n, byte| {
            let byte = number(byte)?;
            if byte > 255 {
                return None;
            }
            n.checked_mul(256)?.checked_add(byte)
        });
    }
    value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .map_or_else(
            || value.parse().ok(),
            |hex| u64::from_str_radix(hex, 16).ok(),
        )
}

fn range(value: &str) -> Option<(u64, u64)> {
    if value.contains(',') {
        return value.split(',').try_fold((0u64, 0u64), |(lo, hi), part| {
            let (a, b) = if part.trim() == "*" {
                (0, 255)
            } else {
                range(part.trim())?
            };
            // A varying prefix needs a full trailing byte to form one interval.
            if b > 255 || (lo != hi && (a != 0 || b != 255)) {
                return None;
            }
            Some((
                lo.checked_mul(256)?.checked_add(a)?,
                hi.checked_mul(256)?.checked_add(b)?,
            ))
        });
    }
    if let Some(n) = number(value) {
        return Some((n, n));
    }
    let (start, end) = value.split_once('-')?;
    let end = if start.starts_with("0x") && !end.starts_with("0x") {
        u64::from_str_radix(end.trim(), 16).ok()?
    } else {
        number(end)?
    };
    let start = number(start)?;
    (start <= end).then_some((start, end))
}

fn render_registry_info(group: &Group, registry: &Registry) -> String {
    let group_id = &group.id;
    let Registry {
        id, title, parent, ..
    } = registry;
    format!(
        "crate::RegistryInfo {{ group: {group_id:?}, id: {id:?}, title: {title:?}, parent: {parent:?} }}"
    )
}

fn status(label: &str) -> &'static str {
    let lower = label.to_ascii_lowercase();
    for (prefix, status) in [
        ("unassigned", "Unassigned"),
        ("reserved for private", "PrivateUse"),
        ("reserved for experimental", "Experimental"),
        ("reserved", "Reserved"),
        ("private use", "PrivateUse"),
        ("experimental", "Experimental"),
    ] {
        if lower.starts_with(prefix) {
            return status;
        }
    }
    "Assigned"
}

fn constant(value: &str, used: &mut BTreeSet<String>) -> String {
    unique(
        ident(value, false)
            .trim_end_matches('_')
            .to_ascii_uppercase(),
        used,
    )
}

fn render_metadata(out: &mut String, group: &Group) {
    writeln!(
        out,
        "#[cfg(feature = \"metadata\")]\npub static REGISTRIES: &[crate::Registry] = &["
    )
    .unwrap();
    for registry in &group.registries {
        let info = render_registry_info(group, registry);
        writeln!(out, "crate::Registry {{ info: {info}, records: &[").unwrap();
        for record in &registry.records {
            writeln!(out, "crate::Record {{ fields: &{record:?} }},").unwrap();
        }
        writeln!(out, "] }},").unwrap();
    }
    writeln!(out, "]; ").unwrap();
}

fn render_registry(
    out: &mut String,
    group: &Group,
    registry: &Registry,
    names: &mut BTreeSet<String>,
) {
    if registry.records.is_empty() {
        return;
    }
    // Infer only a uniform key column. Irregular tables remain in metadata.
    let Some(key) = ["value", "name", "token", "code"].into_iter().find(|key| {
        registry
            .records
            .iter()
            .any(|record| field(record, key).is_some())
    }) else {
        return;
    };
    let Some(values) = registry
        .records
        .iter()
        .map(|record| field(record, key).filter(|value| !value.is_empty()))
        .collect::<Option<Vec<_>>>()
    else {
        return;
    };
    let parsed_ranges: Vec<_> = values.iter().map(|value| range(value)).collect();
    let numeric = parsed_ranges.iter().all(Option::is_some);
    // Mixed numeric/text tables cannot safely be projected as string tokens.
    if !numeric
        && values.iter().zip(&parsed_ranges).any(|(value, range)| {
            range.is_some() || value.parse::<i128>().is_ok() || value.starts_with("0x")
        })
    {
        return;
    }
    let name = unique(
        ident(
            if registry.title.is_empty() {
                &registry.id
            } else {
                &registry.title
            },
            true,
        ),
        names,
    );
    let info = render_registry_info(group, registry);
    let mut constants = BTreeSet::from(["ALL".into(), "REGISTRY".into()]);
    if numeric {
        let max = parsed_ranges
            .iter()
            .flatten()
            .map(|(_, end)| *end)
            .max()
            .unwrap();
        let repr = match max {
            0..=0xff => "u8",
            0x100..=0xffff => "u16",
            0x1_0000..=0xffff_ffff => "u32",
            _ => "u64",
        };
        writeln!(out, "numeric_registry!({name}, {repr}, {info}, [").unwrap();
    } else {
        writeln!(out, "string_registry!({name}, {info}, [").unwrap();
    }
    let mut ranges = String::new();
    for ((record, value), parsed_range) in registry.records.iter().zip(values).zip(parsed_ranges) {
        let label = ["description", "name"]
            .into_iter()
            .filter(|candidate| *candidate != key)
            .find_map(|candidate| field(record, candidate))
            .unwrap_or(value);
        let allocation = status(label);
        if numeric {
            let (start, end) =
                parsed_range.expect("numeric registries have a range for every record");
            writeln!(ranges, "{start} ..= {end} => {allocation},").unwrap();
        }
        if allocation != "Assigned" {
            continue;
        }
        let (name, literal) = if numeric {
            let Some(n) = number(value) else {
                continue;
            };
            (label, format!("{n} => {label:?}"))
        } else {
            (value, format!("{value:?}"))
        };
        let constant = constant(name, &mut constants);
        writeln!(out, "{constant} = {literal},").unwrap();
    }
    if numeric {
        writeln!(out, "], [{ranges}]);").unwrap();
    } else {
        writeln!(out, "]); ").unwrap();
    }
}

const GENERATED_HEADER: &str = "// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0
// @generated by cargo run -p xtask -- iana generate.
";

fn render(groups: &[Group]) -> Result<BTreeMap<String, String>> {
    let mut files = BTreeMap::new();
    let mut out = GENERATED_HEADER.to_owned();
    let mut modules = BTreeSet::new();
    let mut group_table = String::new();
    writeln!(out, "pub const GROUP_COUNT: usize = {};", groups.len()).unwrap();
    writeln!(
        out,
        "pub const REGISTRY_COUNT: usize = {};",
        groups
            .iter()
            .map(|group| group.registries.len())
            .sum::<usize>()
    )
    .unwrap();
    for group in groups {
        let Group { id, title, .. } = group;
        let mut module = ident(
            group.id.strip_suffix("-parameters").unwrap_or(&group.id),
            false,
        );
        if !modules.insert(module.clone()) {
            module = unique(ident(&group.id, false), &mut modules);
        }
        writeln!(group_table, "#[cfg(feature = {id:?})]\ncrate::RegistryGroup {{ id: {id:?}, title: {title:?}, registries: {module}::REGISTRIES }},").unwrap();
        writeln!(
            out,
            "#[cfg(feature = {id:?})]\n#[doc = {title:?}]\npub mod {module};"
        )
        .unwrap();
        files.insert(format!("src/generated/{module}.rs"), render_group(group)?);
    }
    writeln!(out, "#[cfg(feature = \"metadata\")]\npub static GROUPS: &[crate::RegistryGroup] = &[{group_table}];").unwrap();
    files.insert(GENERATED.to_owned(), format_rust_source(&out)?);
    Ok(files)
}

fn render_group(group: &Group) -> Result<String> {
    let mut out = GENERATED_HEADER.to_owned();
    let Group { id, updated, .. } = group;
    writeln!(
        out,
        "pub const ID: &str = {id:?};\npub const UPDATED: &str = {updated:?};"
    )
    .unwrap();
    render_metadata(&mut out, group);
    let mut names = BTreeSet::new();
    for registry in &group.registries {
        render_registry(&mut out, group, registry, &mut names);
    }
    format_rust_source(&out)
}

pub fn generate(mode: Mode) -> Result<()> {
    let groups = load_groups()?;
    let manifest_path = project_path("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)?;
    let expected_manifest = render_features(&manifest, &groups)?;
    let mut files = render(&groups)?;
    files.insert("Cargo.toml".into(), expected_manifest);
    sync_files(&project_path(""), &files, mode)
}

fn sync_files(root: &Path, files: &BTreeMap<String, String>, mode: Mode) -> Result<()> {
    let stale = stale_modules(root, files)?;
    let check = matches!(mode, Mode::Check);
    for (relative, expected) in files {
        let path = root.join(relative);
        if check {
            if fs::read_to_string(&path).ok().as_deref() != Some(expected) {
                return Err(format!(
                    "{} is stale; run cargo run -p xtask -- iana generate",
                    path.display()
                )
                .into());
            }
        } else {
            write_if_changed(&path, expected)?;
        }
    }
    if check && !stale.is_empty() {
        return Err("obsolete generated modules; run cargo run -p xtask -- iana generate".into());
    }
    for path in stale {
        fs::remove_file(path)?;
    }
    let action = if check { "checked" } else { "generated" };
    println!("{action} {} files", files.len());
    Ok(())
}

fn stale_modules(root: &Path, files: &BTreeMap<String, String>) -> Result<Vec<PathBuf>> {
    let directory = root.join("src/generated");
    let entries = match fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(format!("read {}: {error}", directory.display()).into()),
    };
    let mut stale = Vec::new();
    for entry in entries {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let relative = format!(
            "src/generated/{}",
            path.file_name().unwrap().to_string_lossy()
        );
        if !files.contains_key(&relative)
            && fs::read_to_string(&path)?.starts_with(GENERATED_HEADER)
        {
            stale.push(path);
        }
    }
    stale.sort();
    Ok(stale)
}

fn render_features(manifest: &str, groups: &[Group]) -> Result<String> {
    const START: &str = "# BEGIN GENERATED CATALOG FEATURES\n";
    const END: &str = "# END GENERATED CATALOG FEATURES";
    // A source-only checkout can bootstrap the feature table on its first update.
    let template;
    let manifest = if manifest.contains(START)
        || manifest.lines().any(|line| line.trim() == "[features]")
    {
        manifest
    } else {
        template = format!("{manifest}\n[features]\ndefault = []\nmetadata = []\n{START}{END}\n");
        &template
    };
    let (prefix, rest) = manifest
        .split_once(START)
        .ok_or("missing generated feature start marker")?;
    let (_, suffix) = rest
        .split_once(END)
        .ok_or("missing generated feature end marker")?;
    let mut out = format!("{prefix}{START}");
    for group in groups {
        if ["default", "metadata"].contains(&group.id.as_str()) {
            return Err(format!("catalog ID conflicts with reserved feature: {}", group.id).into());
        }
        writeln!(out, "{:?} = []", group.id).unwrap();
    }
    write!(out, "{END}{suffix}").unwrap();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::parse_snapshot;

    fn fixture(body: &str) -> String {
        format!(
            r#"<registry xmlns="http://www.iana.org/assignments" id="example-parameters"><title>Example</title>{body}</registry>"#
        )
    }

    #[test]
    fn preserves_nested_registries_and_irregular_records() {
        let xml = fixture(
            r#"<registry id="parent"><title>Parent</title><registry id="child"><title>Child</title><record date="2026-01-01"><odd>one <b>two</b> three</odd><odd>four</odd><xref type="rfc" data="rfc123"/></record></registry></registry>"#,
        );
        let group = parse_snapshot(&xml).unwrap();
        assert_eq!(group.registries.len(), 3);
        let child = &group.registries[2];
        assert_eq!(child.parent.as_deref(), Some("parent"));
        assert_eq!(child.records[0][0], ("odd".into(), "one two three".into()));
        assert_eq!(child.records[0][1], ("odd".into(), "four".into()));
        let files = render(std::slice::from_ref(&group)).unwrap();
        let source = files.values().cloned().collect::<String>();
        assert!(source.contains("pub mod example"));
        assert!(source.contains("records:"));
        assert!(!source.contains("xml:"));
        assert!(!source.contains("<registry"));
        assert!(!source.contains("string_registry!"));
    }
}
