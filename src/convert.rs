use std::borrow::Cow;

use leptos::prelude::{Get, Signal};
use reactive_graph::signal::ReadSignal;

use crate::{
    IntoStyleProperty, Style, StyleEntry, StyleProperty, StyleValue, Styles,
    css::{CssDimension, CssDimensionExpr, CssValue},
};

/// Trait for types that can be used as always-present CSS style values in the
/// `.with(property, value)` and `.add(property, value)` methods on [`Styles`].
///
/// This enables the ergonomic syntax `.with(Position, "absolute")` instead of
/// `.with((Position, "absolute"))`.
pub trait IntoStyleEntryValue {
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry;
}

/// Trait for types that can be used as conditionally-present CSS style values in the
/// `.with_optional(property, value)` and `.add_optional(property, value)` methods on [`Styles`].
///
/// When the value resolves to `None`, the style entry is excluded from the output.
pub trait IntoOptionalStyleEntryValue {
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry;
}

/// Parses a style string like "color: red" into property and value parts.
fn parse_style_string(s: &str) -> (Cow<'static, str>, Cow<'static, str>) {
    if let Some((property, value)) = s.split_once(':') {
        (
            Cow::Owned(property.trim().to_string()),
            Cow::Owned(value.trim().trim_end_matches(';').to_string()),
        )
    } else {
        tracing::warn!(
            "Style string '{}' missing ':'. Expected 'property: value'",
            s
        );
        (Cow::Owned(s.trim().to_string()), Cow::Owned(String::new()))
    }
}

/// Generates `From<(P, $val)> for StyleEntry` for all property types via `IntoStyleProperty`.
macro_rules! impl_style_entry_always {
    ($val:ty) => {
        impl<P: IntoStyleProperty> From<(P, $val)> for StyleEntry {
            fn from((p, v): (P, $val)) -> Self {
                StyleEntry::always(p.into_property(), v)
            }
        }
    };
}

impl_style_entry_always!(&'static str);
impl_style_entry_always!(String);
impl_style_entry_always!(CssValue);
impl_style_entry_always!(CssDimension);
impl_style_entry_always!(CssDimensionExpr);

impl From<&'static str> for StyleEntry {
    fn from(s: &'static str) -> Self {
        let (property, value) = parse_style_string(s);
        StyleEntry::always(property, value)
    }
}

impl From<String> for StyleEntry {
    fn from(s: String) -> Self {
        let (property, value) = parse_style_string(&s);
        StyleEntry::always(property, value)
    }
}

macro_rules! impl_style_entry_reactive {
    ($prop:ty, $to_prop:expr) => {
        impl From<($prop, Signal<Option<StyleValue>>)> for StyleEntry {
            fn from((p, v): ($prop, Signal<Option<StyleValue>>)) -> Self {
                StyleEntry::reactive($to_prop(p), v)
            }
        }

        impl From<($prop, Signal<Option<String>>)> for StyleEntry {
            fn from((p, v): ($prop, Signal<Option<String>>)) -> Self {
                StyleEntry::reactive_signal(
                    ($to_prop(p)).into(),
                    Signal::derive(move || v.get().map(CssValue::from)),
                )
            }
        }
    };
}

impl_style_entry_reactive!(&'static str, std::convert::identity);
impl_style_entry_reactive!(String, std::convert::identity);

// Style + Signal impls gated on non-nightly due to potential overlap with closure impl.
#[cfg(not(feature = "nightly"))]
impl From<(Style, Signal<Option<StyleValue>>)> for StyleEntry {
    fn from((p, v): (Style, Signal<Option<StyleValue>>)) -> Self {
        StyleEntry::reactive(p.as_str(), v)
    }
}

#[cfg(not(feature = "nightly"))]
impl From<(Style, Signal<Option<String>>)> for StyleEntry {
    fn from((p, v): (Style, Signal<Option<String>>)) -> Self {
        StyleEntry::reactive_signal(
            Cow::Borrowed(p.as_str()),
            Signal::derive(move || v.get().map(CssValue::from)),
        )
    }
}

impl From<(&'static str, ReadSignal<Option<StyleValue>>)> for StyleEntry {
    fn from((property, value): (&'static str, ReadSignal<Option<StyleValue>>)) -> Self {
        let value: Signal<Option<StyleValue>> = value.into();
        StyleEntry::reactive(property, value)
    }
}

impl From<(&'static str, ReadSignal<Option<String>>)> for StyleEntry {
    fn from((property, value): (&'static str, ReadSignal<Option<String>>)) -> Self {
        let value: Signal<Option<StyleValue>> = Signal::derive(move || value.get().map(Into::into));
        StyleEntry::reactive(property, value)
    }
}

macro_rules! impl_style_entry_optional {
    ($val:ty) => {
        impl<P: IntoStyleProperty> From<(P, Option<$val>)> for StyleEntry {
            fn from((p, v): (P, Option<$val>)) -> Self {
                StyleEntry::static_optional(p.into_property(), v.map(CssValue::from))
            }
        }
    };
}

impl_style_entry_optional!(&'static str);
impl_style_entry_optional!(String);

impl<F, V> From<(Style, F)> for StyleEntry
where
    V: Into<StyleValue>,
    F: Fn() -> Option<V> + Send + Sync + 'static,
{
    fn from((property, value_fn): (Style, F)) -> Self {
        StyleEntry::reactive_signal(
            Cow::Borrowed(property.as_str()),
            Signal::derive(move || value_fn().map(Into::into)),
        )
    }
}

macro_rules! impl_style_entry_value_always {
    ($val:ty) => {
        impl IntoStyleEntryValue for $val {
            fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
                StyleEntry::always(property, self)
            }
        }
    };
}

impl_style_entry_value_always!(&'static str);
impl_style_entry_value_always!(String);
impl_style_entry_value_always!(CssValue);
impl_style_entry_value_always!(CssDimension);
impl_style_entry_value_always!(CssDimensionExpr);

impl<F, V> IntoStyleEntryValue for F
where
    V: Into<StyleValue>,
    F: Fn() -> V + Send + Sync + 'static,
{
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
        StyleEntry::reactive_signal(property, Signal::derive(move || Some(self().into())))
    }
}

impl<V: Into<StyleValue>> IntoOptionalStyleEntryValue for Option<V> {
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
        StyleEntry::static_optional(property, self.map(Into::into))
    }
}

// Signal impls gated on non-nightly due to Signal<T>: Fn() -> T on nightly,
// which would overlap with the closure impl below.
#[cfg(not(feature = "nightly"))]
impl IntoOptionalStyleEntryValue for Signal<Option<StyleValue>> {
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
        StyleEntry::reactive(property, self)
    }
}

#[cfg(not(feature = "nightly"))]
impl IntoOptionalStyleEntryValue for Signal<Option<String>> {
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
        StyleEntry::reactive_signal(
            property,
            Signal::derive(move || self.get().map(CssValue::from)),
        )
    }
}

impl IntoOptionalStyleEntryValue for ReadSignal<Option<StyleValue>> {
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
        let value: Signal<Option<StyleValue>> = self.into();
        StyleEntry::reactive(property, value)
    }
}

impl IntoOptionalStyleEntryValue for ReadSignal<Option<String>> {
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
        let value: Signal<Option<StyleValue>> = Signal::derive(move || self.get().map(Into::into));
        StyleEntry::reactive(property, value)
    }
}

impl<F, V> IntoOptionalStyleEntryValue for F
where
    V: Into<StyleValue>,
    F: Fn() -> Option<V> + Send + Sync + 'static,
{
    fn into_style_entry(self, property: StyleProperty) -> StyleEntry {
        StyleEntry::reactive_signal(property, Signal::derive(move || self().map(Into::into)))
    }
}

impl<T: Into<StyleEntry>> From<T> for Styles {
    fn from(entry: T) -> Self {
        Styles::builder().with_entry(entry).build()
    }
}

impl<T: Into<StyleEntry>, const N: usize> From<[T; N]> for Styles {
    fn from(entries: [T; N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}
