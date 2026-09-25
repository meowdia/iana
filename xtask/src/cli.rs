// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Result;
use crate::snapshot::validate_id;
use std::collections::BTreeSet;

const USAGE: &str = "usage: cargo run -p xtask -- iana <discover|fetch [GROUP...|--all]|update [GROUP...|--all]|generate|check>";

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Discover,
    Fetch(Selection),
    Update(Selection),
    Generate,
    Check,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Selection {
    All,
    Groups(Vec<String>),
}

impl Command {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self> {
        let args: Vec<_> = args.into_iter().collect();
        let args: Vec<_> = args.iter().map(String::as_str).collect();
        match args.as_slice() {
            ["iana", "discover"] => Ok(Self::Discover),
            ["iana", "generate"] => Ok(Self::Generate),
            ["iana", "check"] => Ok(Self::Check),
            ["iana", "fetch", groups @ ..] => Ok(Self::Fetch(Selection::parse(groups)?)),
            ["iana", "update", groups @ ..] => Ok(Self::Update(Selection::parse(groups)?)),
            _ => Err(USAGE.into()),
        }
    }
}

impl Selection {
    fn parse(args: &[&str]) -> Result<Self> {
        if args.is_empty() || matches!(args, [flag] if *flag == "--all") {
            return Ok(Self::All);
        }
        let mut groups = BTreeSet::new();
        for group in args {
            if group.starts_with('-') {
                return Err(USAGE.into());
            }
            validate_id(group)?;
            groups.insert((*group).to_owned());
        }
        Ok(Self::Groups(groups.into_iter().collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Command> {
        Command::parse(args.iter().map(|arg| (*arg).to_owned()))
    }

    #[test]
    fn updates_discover_all_groups_by_default() {
        assert_eq!(
            parse(&["iana", "update"]).unwrap(),
            Command::Update(Selection::All)
        );
        assert_eq!(
            parse(&["iana", "fetch", "--all"]).unwrap(),
            Command::Fetch(Selection::All)
        );
    }
}
