use std::{borrow::Cow, fmt, sync::Arc};

use crate::{Styles, UncheckedDeclaration};

/// One declaration that is ready to render, either checked or unchecked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StyleDeclaration {
    Unchecked(UncheckedDeclaration),
    Checked(typed_css::Declaration),
}

impl StyleDeclaration {
    pub(crate) fn property_name(&self) -> &str {
        match self {
            Self::Unchecked(declaration) => declaration.property_name(),
            Self::Checked(declaration) => declaration.property_name(),
        }
    }

    pub(crate) fn value(&self) -> &str {
        match self {
            Self::Unchecked(declaration) => declaration.value(),
            Self::Checked(declaration) => declaration.value(),
        }
    }

    pub(crate) fn write_to(&self, buf: &mut String) {
        buf.push_str(self.property_name());
        buf.push(':');
        buf.push_str(self.value());
        buf.push(';');
    }
}

/// A closure that returns the current declarations. Calling it inside a reactive observer
/// subscribes the observer to the signals that it reads.
type Resolver = Arc<dyn Fn() -> StyleEntry + Send + Sync>;

/// Where the declarations of an entry come from. A static or nested entry always has a
/// declaration, because `Styles` drops a static `None` or an empty `Styles` instead of storing it.
#[derive(Clone)]
enum Source {
    Static(StyleDeclaration),
    Reactive(Resolver),
    /// A whole `Styles`, boxed because `Styles` stores its entries inline.
    Nested(Box<Styles>),
}

impl fmt::Debug for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(declaration) => f.debug_tuple("Static").field(declaration).finish(),
            Self::Reactive(_) => f.write_str("Reactive"),
            Self::Nested(styles) => f.debug_tuple("Nested").field(styles).finish(),
        }
    }
}

/// One entry of [`Styles`](crate::Styles), which is a declaration that may be absent or reactive,
/// or a whole `Styles`.
///
/// Entries of different kinds have the same type, so code can collect them in one list. A checked
/// declaration, an [`UncheckedDeclaration`], a `Styles`, an `Option` of any of them, and a
/// closure that returns one of them all convert into a `StyleEntry`.
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_styles::{
///     StyleEntry, Styles,
///     css::{pct, property::{DISPLAY, WIDTH}},
/// };
///
/// let (wide, _set_wide) = signal(false);
/// let entries = vec![
///     StyleEntry::new(DISPLAY.grid()),
///     StyleEntry::new(move || wide.get().then(|| WIDTH.declare(pct(100)))),
/// ];
/// assert_eq!(Styles::new().with_all(entries).to_style_string(), "display:grid;");
/// ```
#[derive(Clone, Debug)]
pub struct StyleEntry(Option<Source>);

impl StyleEntry {
    /// Create an entry from a declaration, a `Styles`, an `Option` of either, or a closure that
    /// returns one of them.
    ///
    /// [`IntoStyleEntry`] describes the accepted values.
    #[must_use]
    pub fn new<M>(source: impl IntoStyleEntry<M>) -> Self {
        source.into_style_entry()
    }

    /// Return an entry without a declaration.
    pub(crate) const fn empty() -> Self {
        Self(None)
    }

    pub(crate) fn reactive(resolve: impl Fn() -> StyleEntry + Send + Sync + 'static) -> Self {
        Self(Some(Source::Reactive(Arc::new(resolve))))
    }

    /// Visit the current declarations with their rank inside this entry.
    ///
    /// A static entry has one declaration, with an empty rank. A nested `Styles` passes on every
    /// declaration with the rank that it has there, so that a fallback inside the nested `Styles`
    /// stays below the declarations of the entry's own layer. A reactive entry calls its closure,
    /// so the current reactive observer subscribes to the signals that the closure reads.
    pub(crate) fn resolve_each<'a>(
        &'a self,
        visit: &mut dyn FnMut(Cow<'a, StyleDeclaration>, &[u16]),
    ) {
        match &self.0 {
            None => {}
            Some(Source::Static(declaration)) => visit(Cow::Borrowed(declaration), &[]),
            Some(Source::Reactive(resolve)) => resolve().into_each(&mut |declaration, rank| {
                visit(Cow::Owned(declaration), rank);
            }),
            Some(Source::Nested(styles)) => {
                styles.resolve_ranked(&mut |declaration, rank| visit(declaration, &rank));
            }
        }
    }

    /// Visit the current declarations like [`StyleEntry::resolve_each`], but move them out of
    /// the entry. A closure returns a new entry on each call, so its declarations move instead
    /// of being cloned.
    pub(crate) fn into_each(self, visit: &mut dyn FnMut(StyleDeclaration, &[u16])) {
        match self.0 {
            None => {}
            Some(Source::Static(declaration)) => visit(declaration, &[]),
            Some(Source::Reactive(resolve)) => resolve().into_each(visit),
            Some(Source::Nested(styles)) => {
                styles.into_ranked(&mut |declaration, rank| visit(declaration, &rank));
            }
        }
    }

    /// Return whether the entry never has a declaration.
    pub(crate) const fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub(crate) fn is_reactive(&self) -> bool {
        match &self.0 {
            Some(Source::Reactive(_)) => true,
            Some(Source::Nested(styles)) => styles.is_reactive(),
            Some(Source::Static(_)) | None => false,
        }
    }
}

impl From<UncheckedDeclaration> for StyleEntry {
    fn from(declaration: UncheckedDeclaration) -> Self {
        Self(Some(Source::Static(StyleDeclaration::Unchecked(
            declaration,
        ))))
    }
}

impl From<typed_css::Declaration> for StyleEntry {
    fn from(declaration: typed_css::Declaration) -> Self {
        Self(Some(Source::Static(StyleDeclaration::Checked(declaration))))
    }
}

/// A `Styles` converts into one entry. The declarations of its first layer join the layer of the
/// entry, and its fallback layers stay below that layer.
impl From<Styles> for StyleEntry {
    fn from(styles: Styles) -> Self {
        if styles.is_empty() {
            Self::empty()
        } else {
            Self(Some(Source::Nested(Box::new(styles))))
        }
    }
}

/// `None` converts into an entry without a declaration.
impl<D: Into<StyleEntry>> From<Option<D>> for StyleEntry {
    fn from(declaration: Option<D>) -> Self {
        declaration.map_or_else(Self::empty, Into::into)
    }
}

/// A declaration, a `Styles`, an `Option` of either, or a closure that returns one of them.
///
/// [`Styles::with`](crate::Styles::with) and [`StyleEntry::new`] accept these kinds of values:
///
/// - A declaration, such as a checked `Declaration` or an [`UncheckedDeclaration`](crate::UncheckedDeclaration).
///   A [`StyleEntry`] works too.
/// - A whole `Styles`, such as the styles of a variant of a component. The declarations of its
///   first layer join the layer of the entry. Its fallback layers stay below that layer and above
///   the fallback layers of the `Styles` that receives it.
/// - An `Option` of a declaration or a `Styles`. `None` adds nothing.
/// - A closure that returns one of the values above. The closure runs again whenever a signal
///   that it reads changes, so the declarations update reactively. It may return a declaration
///   of another property each time, or `None` while the declaration is absent.
///
/// With the `nightly` feature of Leptos, a signal can be called like a closure. So a signal of a
/// declaration, or of an `Option` of one, is accepted as a reactive declaration, too.
///
/// The type parameter `Marker` only tells these kinds apart. The compiler infers it, so code never
/// names it.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a style declaration",
    label = "expected a declaration, a `Styles`, an `Option` of either, or a closure that returns one of them",
    note = "declare a property from `leptos_styles::css::property`, for example `PADDING.declare(px(16))`",
    note = "for CSS that `typed-css` does not support, wrap the text in an `UncheckedDeclaration`"
)]
pub trait IntoStyleEntry<Marker> {
    /// Convert this value into a style entry.
    fn into_style_entry(self) -> StyleEntry;
}

/// Markers that tell the kinds of [`IntoStyleEntry`] apart. Code never names these types, and the
/// compiler infers them.
pub mod marker {
    use std::marker::PhantomData;

    /// A value that converts into a [`StyleEntry`](super::StyleEntry).
    pub struct Static;

    /// A closure that returns a value of type `D`, which converts into a
    /// [`StyleEntry`](super::StyleEntry).
    pub struct Reactive<D>(PhantomData<D>);
}

impl<D: Into<StyleEntry>> IntoStyleEntry<marker::Static> for D {
    fn into_style_entry(self) -> StyleEntry {
        self.into()
    }
}

impl<F, D> IntoStyleEntry<marker::Reactive<D>> for F
where
    F: Fn() -> D + Send + Sync + 'static,
    D: Into<StyleEntry>,
{
    fn into_style_entry(self) -> StyleEntry {
        StyleEntry::reactive(move || self().into())
    }
}
