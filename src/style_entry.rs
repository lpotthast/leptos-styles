use std::borrow::Cow;

use leptos::prelude::{Get, Signal};

use crate::{
    css::{CssValue, CssWriteTo},
    property::Style,
};

/// A CSS property name that can be either a static string slice or an owned `String`.
pub type StyleProperty = Cow<'static, str>;

/// A CSS value. Alias for [`CssValue`].
pub type StyleValue = CssValue;

pub(crate) fn normalize_style_property(property: StyleProperty) -> StyleProperty {
    let original = property.as_ref();
    let trimmed = original.trim();

    // CSS custom properties are case-sensitive and must not be lowercased.
    if trimmed.starts_with("--") {
        if trimmed == original {
            property
        } else {
            Cow::Owned(trimmed.to_string())
        }
    } else {
        let normalized = trimmed.to_ascii_lowercase();
        if normalized == original {
            property
        } else {
            Cow::Owned(normalized)
        }
    }
}

/// Trait for types that can be used as CSS property names.
pub trait IntoStyleProperty {
    fn into_property(self) -> StyleProperty;
}

impl IntoStyleProperty for &'static str {
    fn into_property(self) -> StyleProperty {
        Cow::Borrowed(self)
    }
}

impl IntoStyleProperty for String {
    fn into_property(self) -> StyleProperty {
        Cow::Owned(self)
    }
}

impl IntoStyleProperty for Style {
    fn into_property(self) -> StyleProperty {
        Cow::Borrowed(self.as_str())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum StyleValueSource {
    Static(Option<StyleValue>),
    Reactive(Signal<Option<StyleValue>>),
}

impl StyleValueSource {
    fn with_current_value<T>(&self, f: impl FnOnce(Option<&StyleValue>) -> T) -> T {
        match self {
            Self::Static(value) => f(value.as_ref()),
            Self::Reactive(signal) => {
                let value = signal.get();
                f(value.as_ref())
            }
        }
    }

    fn is_same_static_value(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(left), Self::Static(right)) => left == right,
            _ => false,
        }
    }

    fn is_static_some(&self) -> bool {
        matches!(self, Self::Static(Some(_)))
    }

    fn touch_if_reactive(&self) {
        if let Self::Reactive(signal) = self {
            let _ = signal.get();
        }
    }
}

/// A single style entry consisting of a CSS property and its reactive optional value.
///
/// When the value signal returns `None`, the style entry is excluded from the output.
#[derive(Clone, Debug)]
pub struct StyleEntry {
    property: StyleProperty,
    value: StyleValueSource,
}

impl StyleEntry {
    pub(crate) fn static_optional(property: StyleProperty, value: Option<StyleValue>) -> Self {
        Self {
            property: normalize_style_property(property),
            value: StyleValueSource::Static(value),
        }
    }

    pub(crate) fn reactive_signal(
        property: StyleProperty,
        value: Signal<Option<StyleValue>>,
    ) -> Self {
        Self {
            property: normalize_style_property(property),
            value: StyleValueSource::Reactive(value),
        }
    }

    /// Creates a style entry with a reactive optional value.
    ///
    /// When the signal returns `None`, this style will be excluded from the output.
    pub fn reactive(
        property: impl Into<StyleProperty>,
        value: impl Into<Signal<Option<StyleValue>>>,
    ) -> Self {
        Self::reactive_signal(property.into(), value.into())
    }

    /// Creates a style entry with a static always-present value.
    pub fn always(property: impl Into<StyleProperty>, value: impl Into<StyleValue>) -> Self {
        Self::static_optional(property.into(), Some(value.into()))
    }

    /// Returns a reference to the CSS property name.
    pub fn property(&self) -> &str {
        &self.property
    }

    pub(crate) fn is_same_static_value(&self, other: &Self) -> bool {
        self.value.is_same_static_value(&other.value)
    }

    pub(crate) fn is_static_some(&self) -> bool {
        self.value.is_static_some()
    }

    pub(crate) fn touch_reactive_dependencies(&self) {
        self.value.touch_if_reactive();
    }

    pub(crate) fn write_declaration_to(&self, buf: &mut String) -> bool {
        self.value.with_current_value(|value| {
            if let Some(value) = value {
                buf.push_str(&self.property);
                buf.push(':');
                value.write_to(buf);
                buf.push(';');
                true
            } else {
                false
            }
        })
    }
}
