#![doc = include_str!("../README.md")]

mod convert;
mod into_style;
mod style_entry;
mod style_list;
mod styles;

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
    pub use ::leptos_css::{
        BorderCornerRadius, CheckedCssValue, CssAngle, CssColor, CssColorName, CssCustomProperty,
        CssDimension, CssDimensionExpr, CssEnvironmentVariable, CssLength, CssTime, CssValue,
        CssVariableReference, CssWriteTo, DeclarationValue, FiniteF64, FontWeight,
        ForcedColorAdjust, Gap, GapValue, GlobalKeyword, Inset, InsetAxis, InvalidCssNumber,
        InvalidCustomPropertyName, InvalidNonNegativeLengthPercentage, InvalidViewTransitionName,
        LengthPercentageAuto, LengthPercentageCalculation, Margin, MarginAxis, MaxSize,
        NonNegativeFiniteF64, NonNegativeLengthPercentage, NonNegativeLengthPercentageValue,
        Opacity, Padding, PaddingAxis, PercentageChannel, PrintColorAdjust, Size, TouchAction,
        TouchActionGestures, TouchActionHorizontalPan, TouchActionVerticalPan, UnitInterval,
        ViewTransitionName, ZIndex, ch, cqh, cqw, css_clamp, css_env, css_max, css_min, deg, dvh,
        dvw, em, fr, grad, hsl, hsla, lvh, lvw, ms, number, pct, px, rad, rem, rgb, rgba, s, svh,
        svw, try_ch, try_cqh, try_cqw, try_deg, try_dvh, try_dvw, try_em, try_fr, try_grad,
        try_hsl, try_hsla, try_lvh, try_lvw, try_ms, try_number, try_pct, try_px, try_rad, try_rem,
        try_rgba, try_s, try_svh, try_svw, try_turn, try_vh, try_vmax, try_vmin, try_vw, turn, var,
        vh, vmax, vmin, vw,
    };
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
///     PaddingProperty.declare(CssColor::Named(CssColorName::Red)),
/// );
/// ```
///
/// ```compile_fail
/// use leptos_styles::Styles;
///
/// // Raw strings require the explicitly named unchecked API.
/// let _ = Styles::builder().with("padding");
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
