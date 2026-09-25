// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Generated, allocation-free IANA registry bindings.
//!
//! Catalogs are opt-in features using canonical IANA IDs, e.g. `sdp-parameters`.
//! `all-registries` enables every catalog; `metadata` adds data only for enabled catalogs.
#![no_std]

/// Static identity of an IANA registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegistryInfo {
    pub group: &'static str,
    pub id: &'static str,
    pub title: &'static str,
    pub parent: Option<&'static str>,
}

/// A registry with its original XML, including fields, references and rules.
#[cfg(feature = "metadata")]
#[derive(Debug)]
pub struct Registry {
    pub info: RegistryInfo,
    pub xml: &'static str,
    pub records: &'static [Record],
}

/// One IANA XML group in the complete discovered catalog.
#[cfg(feature = "metadata")]
#[derive(Debug)]
pub struct RegistryGroup {
    pub id: &'static str,
    pub title: &'static str,
    pub registries: &'static [Registry],
}

/// An XML record's columns, in source order. Repeated columns are retained.
#[cfg(feature = "metadata")]
#[derive(Debug)]
pub struct Record {
    pub fields: &'static [(&'static str, &'static str)],
}

#[cfg(feature = "metadata")]
impl Record {
    pub fn field(&self, name: &str) -> Option<&'static str> {
        self.fields
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| *value)
    }
}

/// Allocation classification recorded by IANA, independent of deprecation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Assigned,
    Unassigned,
    Reserved,
    PrivateUse,
    Experimental,
    Unknown,
}

// A snapshot collection may contain only one kind of typed registry.
#[allow(unused_macros, unused_macro_rules)]
macro_rules! string_registry {
    ($name:ident, $info:expr, [$($constant:ident = $value:literal),* $(,)?]) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name<'a>(&'a str);
        impl<'a> $name<'a> {
            pub const REGISTRY: crate::RegistryInfo = $info;
            $(pub const $constant: Self = Self($value);)*
            pub const ALL: &'static [$name<'static>] = &[$($name($value)),*];
            pub const fn new(value: &'a str) -> Self { Self(value) }
            pub const fn as_str(self) -> &'a str { self.0 }
            pub fn is_registered(self) -> bool { Self::ALL.iter().any(|entry| entry.0 == self.0) }
        }
        impl<'a> From<&'a str> for $name<'a> {
            fn from(value: &'a str) -> Self { Self::new(value) }
        }
        impl core::fmt::Display for $name<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str(self.0) }
        }
    };
}

#[allow(unused_macros, unused_macro_rules)]
macro_rules! numeric_registry {
    ($name:ident, $repr:ty, $info:expr, [$($constant:ident = $value:literal => $label:literal),* $(,)?], [$($start:literal ..= $end:literal => $status:ident),* $(,)?]) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name($repr);
        impl $name {
            pub const REGISTRY: crate::RegistryInfo = $info;
            $(pub const $constant: Self = Self($value);)*
            pub const ALL: &'static [Self] = &[$(Self::$constant),*];
            pub const fn new(value: $repr) -> Self { Self(value) }
            pub const fn value(self) -> $repr { self.0 }
            pub const fn name(self) -> Option<&'static str> {
                $(if self.0 == $value { return Some($label); })*
                None
            }
            pub const fn is_registered(self) -> bool { self.name().is_some() }
            pub fn status(self) -> crate::Status {
                if self.is_registered() { return crate::Status::Assigned; }
                $(if ($start..=$end).contains(&self.0) { return crate::Status::$status; })*
                crate::Status::Unknown
            }
        }
        impl From<$repr> for $name { fn from(value: $repr) -> Self { Self::new(value) } }
        impl From<$name> for $repr { fn from(value: $name) -> Self { value.value() } }
    };
}

mod generated;
pub use generated::*;
