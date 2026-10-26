#![doc = include_str!("../README.md")]

mod into_style;
mod style_entry;
mod styles;
mod unchecked;

pub use style_entry::{IntoStyleEntry, StyleEntry};
pub use styles::Styles;
pub use unchecked::{InvalidUncheckedDeclaration, InvalidUncheckedReason, UncheckedDeclaration};

/// The [`typed-css`](https://crates.io/crates/typed-css) crate, which provides the checked values,
/// properties, and declarations.
pub use ::typed_css as css;

// Styles are passed through component props, which must be `Send` and `Sync`. These assertions
// make a change that loses either bound fail here.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Styles>();
    assert_send_sync::<StyleEntry>();
    assert_send_sync::<UncheckedDeclaration>();
    assert_send_sync::<InvalidUncheckedDeclaration>();
    assert_send_sync::<InvalidUncheckedReason>();
};

/// ```compile_fail
/// use leptos_styles::{Styles, css::{NamedColor, property::PADDING}};
///
/// // `padding` does not accept a color.
/// let _ = Styles::new().with(PADDING.declare(NamedColor::Red));
/// ```
///
/// ```compile_fail
/// use leptos_styles::Styles;
///
/// // CSS text needs an explicit `UncheckedDeclaration`.
/// let _ = Styles::new().with("padding");
/// ```
///
/// ```compile_fail
/// use leptos_styles::Styles;
///
/// // A pair of strings does not convert into a declaration.
/// let _: Styles = ("padding", "16px").into();
/// ```
const _: () = ();
