// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    Result,
    output::{GENERATED, format_rust_source, project_path, write_if_changed},
    snapshot::{Group, Registry, field, load_groups, validate_id},
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

struct RegistryExample {
    name: String,
    assertion: String,
}

fn render_registry(
    out: &mut String,
    group: &Group,
    registry: &Registry,
    names: &mut BTreeSet<String>,
) -> Option<RegistryExample> {
    if registry.records.is_empty() {
        return None;
    }
    // Irregular tables remain in metadata.
    let key = ["value", "name", "token", "code"].into_iter().find(|key| {
        registry
            .records
            .iter()
            .any(|record| field(record, key).is_some())
    })?;
    let values = registry
        .records
        .iter()
        .map(|record| field(record, key).filter(|value| !value.is_empty()))
        .collect::<Option<Vec<_>>>()?;
    let parsed_ranges: Vec<_> = values.iter().map(|value| range(value)).collect();
    let numeric = parsed_ranges.iter().all(Option::is_some);
    // Mixed numeric/text tables cannot safely be projected as string tokens.
    if !numeric
        && values.iter().zip(&parsed_ranges).any(|(value, range)| {
            range.is_some() || value.parse::<i128>().is_ok() || value.starts_with("0x")
        })
    {
        return None;
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
        writeln!(out, "iana::numeric_registry!({name}, {repr}, {info}, [").unwrap();
    } else {
        writeln!(out, "iana::string_registry!({name}, {info}, [").unwrap();
    }
    let mut ranges = String::new();
    let mut example = None;
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
        let (constant_name, literal, example_value, method) = if numeric {
            let Some(n) = number(value) else {
                continue;
            };
            (label, format!("{n} => {label:?}"), n.to_string(), "value")
        } else {
            (value, format!("{value:?}"), format!("{value:?}"), "as_str")
        };
        let constant = constant(constant_name, &mut constants);
        writeln!(out, "{constant} = {literal},").unwrap();
        example.get_or_insert_with(|| RegistryExample {
            name: name.clone(),
            assertion: format!("assert_eq!({name}::{constant}.{method}(), {example_value});"),
        });
    }
    if numeric {
        writeln!(out, "], [{ranges}]);").unwrap();
    } else {
        writeln!(out, "]); ").unwrap();
    }
    example
}

const GENERATED_HEADER: &str = "// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0
// @generated by cargo run -p xtask -- iana generate.
";

struct Assignment {
    family: String,
    optional: bool,
}

type Assignments = BTreeMap<String, Assignment>;

fn parse_assignments(source: &str) -> Result<Assignments> {
    let mut assignments = BTreeMap::new();
    for (index, line) in source.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        let [name, id, availability] = fields.as_slice() else {
            return Err(format!(
                "iana/families.txt:{}: expected crate, catalog ID, and availability",
                index + 1
            )
            .into());
        };
        let family = name
            .strip_prefix("iana-")
            .ok_or("family crate must start with iana-")?;
        if family.is_empty()
            || !family
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            || *name == "iana-gen-shared"
        {
            return Err(format!("invalid or reserved family crate: {name}").into());
        }
        validate_id(id)?;
        let optional = match *availability {
            "always" => false,
            "feature" => true,
            _ => return Err(format!("invalid availability for {id}: {availability}").into()),
        };
        if optional && ["default", "metadata"].contains(id) {
            return Err(format!("catalog ID conflicts with reserved feature: {id}").into());
        }
        if assignments
            .insert(
                (*id).into(),
                Assignment {
                    family: family.into(),
                    optional,
                },
            )
            .is_some()
        {
            return Err(format!("duplicate family assignment for {id}").into());
        }
    }
    if assignments.is_empty() {
        return Err("empty family assignments".into());
    }
    Ok(assignments)
}

fn render(groups: &[Group], assignments: &Assignments) -> Result<BTreeMap<String, String>> {
    let mut families: BTreeMap<&str, Vec<&Group>> = BTreeMap::new();
    for group in groups {
        validate_id(&group.id)?;
        let prefix = assignments
            .get(&group.id)
            .ok_or_else(|| {
                format!(
                    "catalog {} needs an explicit assignment in iana/families.txt",
                    group.id
                )
            })?
            .family
            .as_str();
        families.entry(prefix).or_default().push(group);
    }
    let mut files = BTreeMap::new();
    let mut crate_names = BTreeSet::new();
    let family_lib = format_rust_source(&format!(
        "{GENERATED_HEADER}//! Type-safe, allocation-free bindings to IANA protocol registries.\n#![no_std]\npub use iana::{{RegistryInfo, Status}};\n#[cfg(feature = \"metadata\")]\npub use iana::{{Record, Registry, RegistryGroup}};\nmod generated;\npub use generated::*;\n"
    ))?;
    for (prefix, members) in &families {
        let name = format!("iana-{prefix}");
        if !crate_names.insert(ident(&name, false)) {
            return Err(format!("family crate name collision: {name}").into());
        }
        if members
            .iter()
            .filter(|group| assignments[&group.id].optional)
            .count()
            + 2
            > 300
        {
            return Err(format!("{name} exceeds 300 features").into());
        }
        let family_files = render_family(members, assignments)?;
        files.insert(
            format!("crates/{prefix}/README.md"),
            render_readme(&name, members),
        );
        for (relative, source) in family_files {
            files.insert(format!("crates/{prefix}/{relative}"), source);
        }
        files.insert(
            format!("crates/{prefix}/Cargo.toml"),
            render_manifest(prefix, members, assignments)?,
        );
        files.insert(format!("crates/{prefix}/src/lib.rs"), family_lib.clone());
    }
    files.insert(GENERATED.into(), format!("{GENERATED_HEADER}pub const FAMILY_COUNT: usize = {};\npub const GROUP_COUNT: usize = {};\npub const REGISTRY_COUNT: usize = {};\n", families.len(), groups.len(), groups.iter().map(|group| group.registries.len()).sum::<usize>()));
    Ok(files)
}

const README_HEADER: &str = "<!--\nSPDX-FileCopyrightText: 2026 Meowdia Community\nSPDX-License-Identifier: MIT OR Apache-2.0\n@generated by cargo run -p xtask -- iana generate.\n-->\n\n";

fn catalog_modules<'a>(groups: &[&'a Group]) -> Vec<(&'a Group, String)> {
    let mut used = BTreeSet::new();
    groups
        .iter()
        .map(|group| {
            let mut module = ident(
                group.id.strip_suffix("-parameters").unwrap_or(&group.id),
                false,
            );
            if !used.insert(module.clone()) {
                module = unique(ident(&group.id, false), &mut used);
            }
            (*group, module)
        })
        .collect()
}

fn render_readme(name: &str, groups: &[&Group]) -> String {
    let modules = catalog_modules(groups);
    let prefix = name.strip_prefix("iana-").unwrap_or(name);
    let (example, module) = modules
        .iter()
        .find(|(group, _)| group.id.strip_suffix("-parameters").unwrap_or(&group.id) == prefix)
        .unwrap_or(&modules[0]);
    let feature = &example.id;
    let import = ident(name, false);
    let version = env!("CARGO_PKG_VERSION");
    let usage = readme_usage(example, module, &import);
    let dependency = if groups.len() == 1 {
        format!("{name} = \"{version}\"")
    } else {
        format!("{name} = {{ version = \"{version}\", features = [{feature:?}] }}")
    };
    let mut out = format!(
        "{README_HEADER}# {name}\n\nType-safe, allocation-free Rust bindings for IANA `{prefix}` registries.\nSupports `no_std`.\n\n## Usage\n\n```toml\n[dependencies]\n{dependency}\n```\n\n```rust\n{usage}```\n"
    );
    if groups.len() == 1 {
        write!(out, "\n## Metadata\n\nEnable `metadata` to access `{module}::REGISTRIES` and the crate's `GROUPS`.\n").unwrap();
    } else {
        out.push_str("\n## Features\n\nNo catalogs are enabled by default. Add `metadata` alongside catalog features\nto access their `REGISTRIES` and the crate's `GROUPS`.\n\n| Feature | Module |\n|---|---|\n");
        for (group, module) in modules {
            let id = &group.id;
            writeln!(
                out,
                "| [`{id}`](https://www.iana.org/assignments/{id}/) | `{module}` |"
            )
            .unwrap();
        }
    }
    out.push_str("\n## License\n\nMIT OR Apache-2.0, IANA data retains the IANA/IETF Trust attribution and CC0-1.0.\n");
    out
}

fn readme_usage(group: &Group, module: &str, import: &str) -> String {
    let mut names = BTreeSet::new();
    for registry in &group.registries {
        let mut source = String::new();
        if let Some(RegistryExample { name, assertion }) =
            render_registry(&mut source, group, registry, &mut names)
        {
            return format!("use {import}::{module}::{name};\n\n{assertion}\n");
        }
    }
    format!(
        "use {import}::{module};\n\nassert_eq!({module}::ID, {:?});\n",
        group.id
    )
}

fn render_family(groups: &[&Group], assignments: &Assignments) -> Result<BTreeMap<String, String>> {
    let mut files = BTreeMap::new();
    let mut out = GENERATED_HEADER.to_owned();
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
    for (group, module) in catalog_modules(groups) {
        let Group { id, title, .. } = group;
        let gate = if assignments[id].optional {
            format!("#[cfg(feature = {id:?})]\n")
        } else {
            String::new()
        };
        writeln!(group_table, "{gate}crate::RegistryGroup {{ id: {id:?}, title: {title:?}, registries: {module}::REGISTRIES }},").unwrap();
        writeln!(out, "{gate}#[doc = {title:?}]\npub mod {module};").unwrap();
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
        let _ = render_registry(&mut out, group, registry, &mut names);
    }
    format_rust_source(&out)
}

fn select_groups(
    groups: Vec<Group>,
    assignments: &Assignments,
    selection: &str,
) -> Result<Vec<Group>> {
    let mut selected = BTreeSet::new();
    for name in selection
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let name = name
            .strip_prefix("iana-")
            .ok_or_else(|| format!("invalid crate in iana/crates.txt: {name}"))?;
        if !assignments
            .values()
            .any(|assignment| assignment.family == name)
        {
            return Err(format!("unknown crate in iana/crates.txt: {name}").into());
        }
        if !selected.insert(name) {
            return Err(format!("duplicate crate in iana/crates.txt: {name}").into());
        }
    }
    if selected.is_empty() {
        return Err("empty crate selection in iana/crates.txt".into());
    }
    let mut filtered = Vec::new();
    let mut found = BTreeSet::new();
    for group in groups {
        let assignment = assignments.get(&group.id).ok_or_else(|| {
            format!(
                "catalog {} needs an explicit assignment in iana/families.txt",
                group.id
            )
        })?;
        if selected.contains(assignment.family.as_str()) {
            found.insert(assignment.family.as_str());
            filtered.push(group);
        }
    }
    if let Some(missing) = selected.difference(&found).next() {
        return Err(format!("selected crate {missing} has no snapshots").into());
    }
    Ok(filtered)
}

pub fn generate(mode: Mode) -> Result<()> {
    let assignments = parse_assignments(&fs::read_to_string(project_path("iana/families.txt"))?)?;
    let groups = select_groups(
        load_groups()?,
        &assignments,
        &fs::read_to_string(project_path("iana/crates.txt"))?,
    )?;
    let manifest_path = project_path("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)?;
    let readme = fs::read_to_string(project_path("README.md"))?;
    let expected_readme = render_readme_tree(&readme, &groups, &assignments)?;
    let mut files = render(&groups, &assignments)?;
    preserve_package_versions(&project_path(""), &mut files)?;
    let expected_manifest = render_members(&manifest, &groups, &assignments)?;
    files.insert("Cargo.toml".into(), expected_manifest);
    files.insert("README.md".into(), expected_readme);
    sync_files(&project_path(""), &files, mode)
}

fn preserve_package_versions(root: &Path, files: &mut BTreeMap<String, String>) -> Result<()> {
    for (path, contents) in files.iter_mut() {
        if !path.starts_with("crates/")
            || !(path.ends_with("Cargo.toml") || path.ends_with("README.md"))
        {
            continue;
        }
        let manifest = root.join(path).with_file_name("Cargo.toml");
        let previous = match fs::read_to_string(manifest) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error.into()),
        };
        if let Some(version) = crate::versions::package_version(&previous) {
            *contents = contents.replace(
                "version.workspace = true",
                &format!("version = \"{version}\""),
            );
            if path.ends_with("README.md") {
                *contents = contents.replace(
                    &format!("\"{}\"", env!("CARGO_PKG_VERSION")),
                    &format!("\"{version}\""),
                );
            }
        }
        if let Some(dependency) = previous.lines().find(|line| line.starts_with("iana = ")) {
            *contents = contents.replace("iana.workspace = true", dependency);
        }
    }
    Ok(())
}

fn render_readme_tree(readme: &str, groups: &[Group], assignments: &Assignments) -> Result<String> {
    const START: &str = "<!-- BEGIN GENERATED CRATE TREE -->\n";
    const END: &str = "<!-- END GENERATED CRATE TREE -->";
    let (before, rest) = readme
        .split_once(START)
        .ok_or("missing README crate tree start marker")?;
    let (_, after) = rest
        .split_once(END)
        .ok_or("missing README crate tree end marker")?;
    let mut families: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for group in groups {
        let assignment = assignments.get(&group.id).ok_or_else(|| {
            format!(
                "catalog {} needs an explicit assignment in iana/families.txt",
                group.id
            )
        })?;
        families
            .entry(&assignment.family)
            .or_default()
            .insert(&group.id);
    }
    let mut out =
        format!("{before}{START}\n```text\niana\n├── iana-gen-shared (shared types and macros)\n");
    for (index, (family, catalogs)) in families.iter().enumerate() {
        let last_family = index + 1 == families.len();
        let branch = if last_family {
            "└──"
        } else {
            "├──"
        };
        let indent = if last_family { "    " } else { "│   " };
        writeln!(out, "{branch} iana-{family}").unwrap();
        if catalogs.len() == 1 {
            continue;
        }
        for (index, id) in catalogs.iter().enumerate() {
            let branch = if index + 1 == catalogs.len() {
                "└──"
            } else {
                "├──"
            };
            writeln!(out, "{indent}{branch} {id}").unwrap();
        }
    }
    write!(out, "```\n\n{END}{after}").unwrap();
    Ok(out)
}

fn sync_files(root: &Path, files: &BTreeMap<String, String>, mode: Mode) -> Result<()> {
    let stale = stale_modules(root, files)?;
    let check = matches!(mode, Mode::Check);
    // Write the workspace manifest after its new members exist on disk.
    for (relative, expected) in files
        .iter()
        .filter(|(path, _)| path.as_str() != "Cargo.toml")
        .chain(files.get_key_value("Cargo.toml"))
    {
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
    let mut stale = Vec::new();
    let mut directories = vec![root.join("src/generated"), root.join("crates")];
    let manifest_header = GENERATED_HEADER.replace("//", "#");
    while let Some(directory) = directories.pop() {
        if !directory.exists() {
            continue;
        }
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                if entry.file_name() != "target" {
                    directories.push(path);
                }
                continue;
            }
            if !entry.file_type()?.is_file()
                || !(path.extension().is_some_and(|extension| extension == "rs")
                    || entry.file_name() == "Cargo.toml"
                    || entry.file_name() == "README.md")
            {
                continue;
            }
            let relative = path.strip_prefix(root)?.to_string_lossy();
            if !files.contains_key(relative.as_ref()) {
                let source = fs::read_to_string(&path)?;
                if source.starts_with(GENERATED_HEADER)
                    || source.starts_with(&manifest_header)
                    || source.starts_with(README_HEADER)
                {
                    stale.push(path);
                }
            }
        }
    }
    stale.sort();
    Ok(stale)
}

fn render_members(manifest: &str, groups: &[Group], assignments: &Assignments) -> Result<String> {
    const START: &str = "    # BEGIN GENERATED FAMILY MEMBERS\n";
    const END: &str = "    # END GENERATED FAMILY MEMBERS";
    let (prefix, rest) = manifest
        .split_once(START)
        .ok_or("missing family member start marker")?;
    let (_, suffix) = rest
        .split_once(END)
        .ok_or("missing family member end marker")?;
    let mut out = format!("{prefix}{START}");
    let families: BTreeSet<_> = groups
        .iter()
        .map(|group| assignments[&group.id].family.as_str())
        .collect();
    for prefix in families {
        writeln!(out, "    \"crates/{prefix}\",").unwrap();
    }
    write!(out, "{END}{suffix}").unwrap();
    Ok(out)
}

fn render_manifest(prefix: &str, groups: &[&Group], assignments: &Assignments) -> Result<String> {
    let name = format!("iana-{prefix}");
    let header = GENERATED_HEADER.replace("//", "#");
    let lib = ident(&name, false);
    let mut out = format!(
        r#"{header}
[package]
name = {name:?}
version.workspace = true
edition = "2024"
license = "MIT OR Apache-2.0"
repository.workspace = true
readme = "README.md"
description = "Generated IANA registry bindings for the {prefix} family."

[lib]
name = {lib:?}

[dependencies]
iana.workspace = true

[lints]
workspace = true

[features]
default = []
metadata = ["iana/metadata"]
# BEGIN GENERATED CATALOG FEATURES
"#
    );
    for group in groups
        .iter()
        .filter(|group| assignments[&group.id].optional)
    {
        if ["default", "metadata"].contains(&group.id.as_str()) {
            return Err(format!("catalog ID conflicts with reserved feature: {}", group.id).into());
        }
        writeln!(out, "{:?} = []", group.id).unwrap();
    }
    out.push_str("# END GENERATED CATALOG FEATURES\n");
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
    fn rejects_invalid_assignments_and_unassigned_catalogs() {
        for source in [
            "",
            "iana-ice\tice",
            "iana-ice\tice\tunknown",
            "iana-ice\tice\talways\niana-other\tice\tfeature",
            "iana-../ice\tice\talways",
            "iana-gen-shared\tice\talways",
            "iana-ice\tmetadata\tfeature",
        ] {
            assert!(parse_assignments(source).is_err());
        }
        let assignments = parse_assignments("iana-ice\tice\talways").unwrap();
        assert!(render(&[named_group("ice"), named_group("ice-2")], &assignments).is_err());
    }

    fn named_group(id: &str) -> Group {
        parse_snapshot(&fixture("").replace("example-parameters", id)).unwrap()
    }

    #[test]
    fn adding_an_optional_catalog_preserves_existing_consumers() {
        let root = std::env::temp_dir().join(format!(
            "iana-consumer-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let assignments =
            parse_assignments("iana-ice\tice\talways\niana-ice\tice-2\tfeature").unwrap();
        let mut groups = vec![named_group("ice")];
        let shared = project_path("");
        let manifest = format!(
            r#"[package]
name = "consumer"
version = "0.1.0"
edition = "2024"
[workspace]
members = ["crates/ice"]
[workspace.package]
version = "0.1.0"
repository = "https://github.com/meowdia/iana"
[workspace.dependencies]
iana = {{ package = "iana-gen-shared", path = {shared:?} }}
[workspace.lints]
[dependencies]
iana-ice = {{ path = "crates/ice", features = ["metadata"] }}
"#
        );
        let consumer = "fn main() { assert_eq!(iana_ice::ice::ID, \"ice\"); assert_eq!(iana_ice::GROUPS.len(), 1); }\n";
        for added in [false, true] {
            if added {
                groups.push(named_group("ice-2"));
            }
            let files = render(&groups, &assignments).unwrap();
            sync_files(&root, &files, Mode::Write).unwrap();
            write_if_changed(&root.join("Cargo.toml"), &manifest).unwrap();
            write_if_changed(&root.join("src/main.rs"), consumer).unwrap();
            run_consumer(&root, &[]);
        }
        write_if_changed(&root.join("src/main.rs"), "fn main() { assert_eq!(iana_ice::ice::ID, \"ice\"); assert_eq!(iana_ice::ice_2::ID, \"ice-2\"); assert_eq!(iana_ice::GROUPS.len(), 2); }\n").unwrap();
        run_consumer(&root, &["--features", "iana-ice/ice-2"]);
        let files = render(&[named_group("ice-2")], &assignments).unwrap();
        sync_files(&root, &files, Mode::Write).unwrap();
        write_if_changed(
            &root.join("src/main.rs"),
            "fn main() { assert!(iana_ice::GROUPS.is_empty()); }\n",
        )
        .unwrap();
        run_consumer(&root, &[]);
        fs::remove_dir_all(root).unwrap();
    }

    fn run_consumer(root: &Path, args: &[&str]) {
        let output = std::process::Command::new(env!("CARGO"))
            .args(["run", "--offline", "--quiet"])
            .args(args)
            .current_dir(root)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn rejects_families_above_the_feature_limit() {
        let groups: Vec<_> = (0..299)
            .map(|index| named_group(&format!("http-test-{index}")))
            .collect();
        let assignments = groups
            .iter()
            .map(|group| {
                (
                    group.id.clone(),
                    Assignment {
                        family: "http".into(),
                        optional: true,
                    },
                )
            })
            .collect();
        assert!(render(&groups, &assignments).is_err());
    }

    #[test]
    fn removes_stale_catalog_modules_without_removing_user_files() {
        let root = std::env::temp_dir().join(format!(
            "iana-families-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let assignments = parse_assignments(
            "iana-http\thttp-methods\tfeature\niana-http\thttp3-parameters\tfeature",
        )
        .unwrap();
        let old = render(
            &[named_group("http-methods"), named_group("http3-parameters")],
            &assignments,
        )
        .unwrap();
        sync_files(&root, &old, Mode::Write).unwrap();
        let user = root.join("crates/http/src/custom.rs");
        write_if_changed(&user, "// User-maintained file.\n").unwrap();
        let new = render(&[named_group("http-methods")], &assignments).unwrap();
        assert!(sync_files(&root, &new, Mode::Check).is_err());
        sync_files(&root, &new, Mode::Write).unwrap();
        assert!(!root.join("crates/http/src/generated/http3.rs").exists());
        assert!(user.exists());
        sync_files(&root, &new, Mode::Check).unwrap();
        fs::remove_dir_all(root).unwrap();
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
    }
}
