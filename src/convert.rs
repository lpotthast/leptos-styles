#[cfg(not(feature = "nightly"))]
use leptos::prelude::Get;
use leptos::prelude::Signal;
#[cfg(not(feature = "nightly"))]
use reactive_graph::signal::ReadSignal;

use crate::{
    StyleEntry, Styles, UncheckedPropertyName, UncheckedStyleValue, style_entry::StyleDeclaration,
};

/// Conversion from an optional source into an explicitly unchecked style entry.
///
/// This trait powers [`Styles::add_optional_unchecked`] and
/// [`crate::StylesBuilder::with_optional_unchecked`].
pub trait IntoOptionalUncheckedStyleValue<V> {
    /// Pair the optional value source with an unchecked property name.
    fn into_optional_unchecked_entry(self, property: UncheckedPropertyName) -> StyleEntry;
}

impl<V> IntoOptionalUncheckedStyleValue<V> for Option<V>
where
    V: Into<UncheckedStyleValue>,
{
    fn into_optional_unchecked_entry(self, property: UncheckedPropertyName) -> StyleEntry {
        StyleEntry::static_optional(
            self.map(|value| StyleDeclaration::unchecked(property, value.into())),
        )
    }
}

// Leptos' nightly feature makes Signal implement Fn, which overlaps the closure
// implementation below. On nightly, callers can pass `move || signal.get()`.
#[cfg(not(feature = "nightly"))]
impl<V> IntoOptionalUncheckedStyleValue<V> for Signal<Option<V>>
where
    V: Clone + Send + Sync + 'static + Into<UncheckedStyleValue>,
{
    fn into_optional_unchecked_entry(self, property: UncheckedPropertyName) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self.get()
                .map(|value| StyleDeclaration::unchecked(property.clone(), value.into()))
        }))
    }
}

#[cfg(not(feature = "nightly"))]
impl<V> IntoOptionalUncheckedStyleValue<V> for ReadSignal<Option<V>>
where
    V: Clone + Send + Sync + 'static + Into<UncheckedStyleValue>,
{
    fn into_optional_unchecked_entry(self, property: UncheckedPropertyName) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self.get()
                .map(|value| StyleDeclaration::unchecked(property.clone(), value.into()))
        }))
    }
}

impl<F, V> IntoOptionalUncheckedStyleValue<V> for F
where
    V: Into<UncheckedStyleValue>,
    F: Fn() -> Option<V> + Send + Sync + 'static,
{
    fn into_optional_unchecked_entry(self, property: UncheckedPropertyName) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self().map(|value| StyleDeclaration::unchecked(property.clone(), value.into()))
        }))
    }
}

/// Conversion from an optional complete declaration source into a [`StyleEntry`].
///
/// The declaration can be a parsed [`StyleEntry`] or, with `typed-css`, a
/// `leptos_css::CheckedDeclaration`. Because the source yields the complete declaration,
/// a reactive source may safely change its property as well as its value.
pub trait IntoOptionalStyleDeclaration<D> {
    /// Convert this optional declaration source into a style entry.
    fn into_optional_declaration_entry(self) -> StyleEntry;
}

impl<D> IntoOptionalStyleDeclaration<D> for Option<D>
where
    D: Into<StyleEntry>,
{
    fn into_optional_declaration_entry(self) -> StyleEntry {
        match self {
            Some(declaration) => declaration.into(),
            None => StyleEntry::static_optional(None),
        }
    }
}

#[cfg(not(feature = "nightly"))]
impl<D> IntoOptionalStyleDeclaration<D> for Signal<Option<D>>
where
    D: Clone + Send + Sync + 'static + Into<StyleEntry>,
{
    fn into_optional_declaration_entry(self) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self.get()
                .and_then(|declaration| declaration.into().resolve().map(Cow::into_owned))
        }))
    }
}

#[cfg(not(feature = "nightly"))]
impl<D> IntoOptionalStyleDeclaration<D> for ReadSignal<Option<D>>
where
    D: Clone + Send + Sync + 'static + Into<StyleEntry>,
{
    fn into_optional_declaration_entry(self) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self.get()
                .and_then(|declaration| declaration.into().resolve().map(Cow::into_owned))
        }))
    }
}

impl<F, D> IntoOptionalStyleDeclaration<D> for F
where
    D: Into<StyleEntry>,
    F: Fn() -> Option<D> + Send + Sync + 'static,
{
    fn into_optional_declaration_entry(self) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self().and_then(|declaration| declaration.into().resolve().map(Cow::into_owned))
        }))
    }
}

use std::borrow::Cow;

/// Conversion from an optional value source for one checked property.
///
/// This trait is available only with `typed-css` and powers the property-first
/// `with_optional` and `add_optional` methods.
#[cfg(feature = "typed-css")]
pub trait IntoOptionalPropertyValue<P, V>
where
    P: leptos_css::property::CheckedProperty,
{
    /// Build an optional checked declaration without separating its property and value.
    fn into_optional_property_entry(self, property: P) -> StyleEntry;
}

/// Conversion from an always-present reactive value source for one checked property.
///
/// This trait is available only with `typed-css` and powers the property-first
/// [`Styles::add_reactive`] and [`crate::StylesBuilder::with_reactive`] methods.
#[cfg(feature = "typed-css")]
pub trait IntoReactivePropertyValue<P, V>
where
    P: leptos_css::property::CheckedProperty,
{
    /// Build a reactive checked declaration without separating its property and value.
    fn into_reactive_property_entry(self, property: P) -> StyleEntry;
}

// Leptos' nightly feature makes Signal implement Fn, which overlaps the closure
// implementation below. On nightly, callers can pass `move || signal.get()`.
#[cfg(all(feature = "typed-css", not(feature = "nightly")))]
impl<P, V> IntoReactivePropertyValue<P, V> for Signal<V>
where
    P: leptos_css::property::CheckedProperty,
    V: Clone + Send + Sync + 'static + Into<leptos_css::DeclarationValue<P::Value>>,
{
    fn into_reactive_property_entry(self, property: P) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            Some(StyleDeclaration::Checked(property.declare(self.get())))
        }))
    }
}

#[cfg(all(feature = "typed-css", not(feature = "nightly")))]
impl<P, V> IntoReactivePropertyValue<P, V> for ReadSignal<V>
where
    P: leptos_css::property::CheckedProperty,
    V: Clone + Send + Sync + 'static + Into<leptos_css::DeclarationValue<P::Value>>,
{
    fn into_reactive_property_entry(self, property: P) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            Some(StyleDeclaration::Checked(property.declare(self.get())))
        }))
    }
}

#[cfg(feature = "typed-css")]
impl<P, F, V> IntoReactivePropertyValue<P, V> for F
where
    P: leptos_css::property::CheckedProperty,
    V: Into<leptos_css::DeclarationValue<P::Value>>,
    F: Fn() -> V + Send + Sync + 'static,
{
    fn into_reactive_property_entry(self, property: P) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            Some(StyleDeclaration::Checked(property.declare(self())))
        }))
    }
}

#[cfg(feature = "typed-css")]
impl<P, V> IntoOptionalPropertyValue<P, V> for Option<V>
where
    P: leptos_css::property::CheckedProperty,
    V: Into<leptos_css::DeclarationValue<P::Value>>,
{
    fn into_optional_property_entry(self, property: P) -> StyleEntry {
        StyleEntry::static_optional(
            self.map(|value| StyleDeclaration::Checked(property.declare(value))),
        )
    }
}

#[cfg(all(feature = "typed-css", not(feature = "nightly")))]
impl<P, V> IntoOptionalPropertyValue<P, V> for Signal<Option<V>>
where
    P: leptos_css::property::CheckedProperty,
    V: Clone + Send + Sync + 'static + Into<leptos_css::DeclarationValue<P::Value>>,
{
    fn into_optional_property_entry(self, property: P) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self.get()
                .map(|value| StyleDeclaration::Checked(property.declare(value)))
        }))
    }
}

#[cfg(all(feature = "typed-css", not(feature = "nightly")))]
impl<P, V> IntoOptionalPropertyValue<P, V> for ReadSignal<Option<V>>
where
    P: leptos_css::property::CheckedProperty,
    V: Clone + Send + Sync + 'static + Into<leptos_css::DeclarationValue<P::Value>>,
{
    fn into_optional_property_entry(self, property: P) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self.get()
                .map(|value| StyleDeclaration::Checked(property.declare(value)))
        }))
    }
}

#[cfg(feature = "typed-css")]
impl<P, F, V> IntoOptionalPropertyValue<P, V> for F
where
    P: leptos_css::property::CheckedProperty,
    V: Into<leptos_css::DeclarationValue<P::Value>>,
    F: Fn() -> Option<V> + Send + Sync + 'static,
{
    fn into_optional_property_entry(self, property: P) -> StyleEntry {
        StyleEntry::reactive_signal(Signal::derive(move || {
            self().map(|value| StyleDeclaration::Checked(property.declare(value)))
        }))
    }
}

impl<T> From<T> for Styles
where
    T: Into<StyleEntry>,
{
    fn from(entry: T) -> Self {
        Styles::builder().with_declaration(entry).build()
    }
}

impl<T, const N: usize> From<[T; N]> for Styles
where
    T: Into<StyleEntry>,
{
    fn from(entries: [T; N]) -> Self {
        Styles::builder().with_declarations(entries).build()
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use crate::{StyleEntry, Styles};

    #[test]
    fn parsed_entry_converts_to_styles() {
        let styles: Styles = StyleEntry::parse("color: red").unwrap().into();
        assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());
    }

    #[test]
    fn array_of_explicit_unchecked_entries_renders_in_order() {
        let styles: Styles = [
            StyleEntry::always_unchecked("color", "blue"),
            StyleEntry::always_unchecked("padding", "20px"),
        ]
        .into();
        assert_that!(styles.to_style_string()).is_equal_to("color:blue;padding:20px;".to_string());
    }

    #[cfg(feature = "typed-css")]
    #[test]
    fn checked_declaration_converts_without_losing_pairing() {
        use leptos_css::{CssColor, CssColorName, property::BackgroundColorProperty};

        let styles: Styles = BackgroundColorProperty
            .declare(CssColor::Named(CssColorName::Red))
            .into();
        assert_that!(styles.to_style_string()).is_equal_to("background-color:red;".to_string());
    }
}
