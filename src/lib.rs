#![doc = include_str!("../README.md")]

pub mod css;
pub mod property;

mod convert;
mod into_style;
mod style_entry;
mod style_list;
mod styles;

pub use convert::{IntoOptionalStyleEntryValue, IntoStyleEntryValue};
pub use css::{CssValue, CssWriteTo};
pub use into_style::StylesState;
pub use property::{Style, Style::*};
pub use style_entry::{IntoStyleProperty, StyleEntry, StyleProperty, StyleValue};
/// Internal compatibility re-export for `TypedBuilder`-generated builder signatures.
///
/// `StyleList` remains an implementation detail with no public constructors. Changes to this
/// hidden re-export and the internal type it exposes are not covered by the crate's usual semver
/// expectations.
#[doc(hidden)]
pub use style_list::StyleList;
pub use styles::Styles;

#[cfg(test)]
mod tests;
