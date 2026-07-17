#![doc = include_str!("../README.md")]

mod convert;
mod into_style;
mod style_entry;
mod style_list;
mod styles;

#[cfg(feature = "typed-css")]
pub use convert::{IntoOptionalPropertyValue, IntoReactivePropertyValue};
pub use convert::{IntoOptionalStyleDeclaration, IntoOptionalUncheckedStyleValue};
pub use into_style::StylesState;
pub use style_entry::{
    IntoUncheckedPropertyName, ParseStyleEntryError, StyleEntry, UncheckedPropertyName,
    UncheckedStyleValue,
};
pub use styles::{Styles, StylesBuilder};

/// Grammar-checked CSS values, property selectors, and declarations. Available only when the
/// `typed-css` cargo feature is enabled, which pulls in the [`leptos-css`] crate.
///
/// [`leptos-css`]: https://crates.io/crates/leptos-css
#[cfg(feature = "typed-css")]
pub use ::leptos_css;

#[cfg(feature = "typed-css")]
pub use ::leptos_css::{CheckedDeclaration, css_custom_property};

/// Re-export the typed CSS value primitives from [`leptos_css`].
#[cfg(feature = "typed-css")]
pub mod css {
    pub use crate::css_custom_property;
    pub use ::leptos_css::value::*;
}

/// Re-export the checked CSS property selectors from [`leptos_css`].
#[cfg(feature = "typed-css")]
pub mod property {
    pub use ::leptos_css::property::*;
}

/// Compile-time assertions that the public types stay `Send + Sync`. `Styles` is intended to
/// flow through Leptos component props, which require both bounds. If a future change to any
/// internal field weakens that, this `const _` will fail to compile and surface the regression
/// at the crate boundary instead of at a downstream consumer.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Styles>();
    assert_send_sync::<StylesBuilder>();
    assert_send_sync::<StyleEntry>();
    assert_send_sync::<ParseStyleEntryError>();
};

/// ```compile_fail
/// use leptos_styles::{Styles, css::{CssColor, CssColorName}, property::PaddingProperty};
///
/// // A color grammar cannot be paired with the padding property.
/// let _ = Styles::builder().with(
///     PaddingProperty,
///     CssColor::Named(CssColorName::Red),
/// );
/// ```
///
/// ```compile_fail
/// use leptos_styles::Styles;
///
/// // Raw strings require the explicitly named unchecked API.
/// let _ = Styles::builder().with("padding", "16px");
/// ```
///
/// ```compile_fail
/// use leptos_styles::Styles;
///
/// // Raw property/value tuples are not an implicit conversion boundary.
/// let _: Styles = ("padding", "16px").into();
/// ```
#[cfg(feature = "typed-css")]
const _: () = ();
