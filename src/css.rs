use std::fmt;

use leptos::oco::Oco;

/// Trait for CSS types that can write their representation to any `fmt::Write` target.
///
/// This powers both [`Display`] and the zero-allocation [`write_to`](CssWriteTo::write_to)
/// method through a single implementation.
pub trait CssWriteTo {
    /// Write the CSS representation to a `fmt::Write` target.
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;

    /// Write the CSS representation directly to a `String` buffer.
    ///
    /// This is infallible because `fmt::Write` for `String` never fails.
    fn write_to(&self, buf: &mut String) {
        let _ = self.css_fmt(buf);
    }
}

macro_rules! impl_display_via_css_fmt {
    ($($ty:ty),+ $(,)?) => {$(
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.css_fmt(f)
            }
        }
    )+};
}

/// CSS length values with absolute and relative units.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssLength {
    /// Pixels.
    Px(f64),
    /// Em units (relative to element font size).
    Em(f64),
    /// Rem units (relative to root font size).
    Rem(f64),
    /// Viewport width units.
    Vw(f64),
    /// Viewport height units.
    Vh(f64),
    /// Viewport minimum dimension units.
    Vmin(f64),
    /// Viewport maximum dimension units.
    Vmax(f64),
    /// Character width units (width of the `0` glyph in the element's font).
    Ch(f64),
    /// Dynamic viewport width units (1dvw = 1% of the dynamic viewport width).
    /// Unlike `vw`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    Dvw(f64),
    /// Dynamic viewport height units (1dvh = 1% of the dynamic viewport height).
    /// Unlike `vh`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    Dvh(f64),
    /// Small viewport width units (1svw = 1% of the smallest possible viewport width).
    /// Represents the viewport size when all dynamic browser UI is expanded/visible.
    Svw(f64),
    /// Small viewport height units (1svh = 1% of the smallest possible viewport height).
    /// Represents the viewport size when all dynamic browser UI is expanded/visible.
    Svh(f64),
    /// Large viewport width units (1lvw = 1% of the largest possible viewport width).
    /// Represents the viewport size when all dynamic browser UI is retracted/hidden.
    Lvw(f64),
    /// Large viewport height units (1lvh = 1% of the largest possible viewport height).
    /// Represents the viewport size when all dynamic browser UI is retracted/hidden.
    Lvh(f64),
    /// Container query width units (1cqw = 1% of the nearest size container's width).
    /// Used with CSS container queries (`@container`).
    Cqw(f64),
    /// Container query height units (1cqh = 1% of the nearest size container's height).
    /// Used with CSS container queries (`@container`).
    Cqh(f64),
}

impl CssLength {
    /// Returns the inner numeric value regardless of unit.
    pub fn value(self) -> f64 {
        match self {
            Self::Px(v)
            | Self::Em(v)
            | Self::Rem(v)
            | Self::Vw(v)
            | Self::Vh(v)
            | Self::Vmin(v)
            | Self::Vmax(v)
            | Self::Ch(v)
            | Self::Dvw(v)
            | Self::Dvh(v)
            | Self::Svw(v)
            | Self::Svh(v)
            | Self::Lvw(v)
            | Self::Lvh(v)
            | Self::Cqw(v)
            | Self::Cqh(v) => v,
        }
    }

    /// Returns a new `CssLength` with the same unit but the value transformed by `f`.
    #[must_use]
    pub fn map_value(self, f: impl FnOnce(f64) -> f64) -> Self {
        match self {
            Self::Px(v) => Self::Px(f(v)),
            Self::Em(v) => Self::Em(f(v)),
            Self::Rem(v) => Self::Rem(f(v)),
            Self::Vw(v) => Self::Vw(f(v)),
            Self::Vh(v) => Self::Vh(f(v)),
            Self::Vmin(v) => Self::Vmin(f(v)),
            Self::Vmax(v) => Self::Vmax(f(v)),
            Self::Ch(v) => Self::Ch(f(v)),
            Self::Dvw(v) => Self::Dvw(f(v)),
            Self::Dvh(v) => Self::Dvh(f(v)),
            Self::Svw(v) => Self::Svw(f(v)),
            Self::Svh(v) => Self::Svh(f(v)),
            Self::Lvw(v) => Self::Lvw(f(v)),
            Self::Lvh(v) => Self::Lvh(f(v)),
            Self::Cqw(v) => Self::Cqw(f(v)),
            Self::Cqh(v) => Self::Cqh(f(v)),
        }
    }
}

impl CssWriteTo for CssLength {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Px(v) => write!(w, "{v}px"),
            Self::Em(v) => write!(w, "{v}em"),
            Self::Rem(v) => write!(w, "{v}rem"),
            Self::Vw(v) => write!(w, "{v}vw"),
            Self::Vh(v) => write!(w, "{v}vh"),
            Self::Vmin(v) => write!(w, "{v}vmin"),
            Self::Vmax(v) => write!(w, "{v}vmax"),
            Self::Ch(v) => write!(w, "{v}ch"),
            Self::Dvw(v) => write!(w, "{v}dvw"),
            Self::Dvh(v) => write!(w, "{v}dvh"),
            Self::Svw(v) => write!(w, "{v}svw"),
            Self::Svh(v) => write!(w, "{v}svh"),
            Self::Lvw(v) => write!(w, "{v}lvw"),
            Self::Lvh(v) => write!(w, "{v}lvh"),
            Self::Cqw(v) => write!(w, "{v}cqw"),
            Self::Cqh(v) => write!(w, "{v}cqh"),
        }
    }
}

/// CSS angle values.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssAngle {
    /// Degrees (360deg = full circle).
    Deg(f64),
    /// Radians (2π rad = full circle).
    Rad(f64),
    /// Turns (1turn = full circle).
    Turn(f64),
    /// Gradians (400grad = full circle).
    Grad(f64),
}

impl CssWriteTo for CssAngle {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Deg(v) => write!(w, "{v}deg"),
            Self::Rad(v) => write!(w, "{v}rad"),
            Self::Turn(v) => write!(w, "{v}turn"),
            Self::Grad(v) => write!(w, "{v}grad"),
        }
    }
}

/// CSS time values.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssTime {
    /// Seconds.
    S(f64),
    /// Milliseconds.
    Ms(f64),
}

impl CssWriteTo for CssTime {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::S(v) => write!(w, "{v}s"),
            Self::Ms(v) => write!(w, "{v}ms"),
        }
    }
}

/// CSS color values.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssColor {
    /// RGB color (e.g., `rgb(255, 128, 0)`).
    Rgb(u8, u8, u8),
    /// RGBA color with alpha (e.g., `rgba(255, 128, 0, 0.5)`).
    Rgba(u8, u8, u8, f64),
    /// HSL color (e.g., `hsl(120, 100%, 50%)`).
    /// Hue in degrees (0-360), saturation and lightness as percentages (0-100).
    Hsl(f64, f64, f64),
    /// HSLA color with alpha (e.g., `hsla(120, 100%, 50%, 0.5)`).
    /// Hue in degrees (0-360), saturation and lightness as percentages (0-100), alpha (0.0-1.0).
    Hsla(f64, f64, f64, f64),
    /// A CSS named color (e.g., `"red"`, `"transparent"`, `"currentColor"`).
    Named(&'static str),
}

impl CssWriteTo for CssColor {
    #[allow(clippy::many_single_char_names)]
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Rgb(r, g, b) => write!(w, "rgb({r}, {g}, {b})"),
            Self::Rgba(r, g, b, a) => write!(w, "rgba({r}, {g}, {b}, {a})"),
            Self::Hsl(h, s, l) => write!(w, "hsl({h}, {s}%, {l}%)"),
            Self::Hsla(h, s, l, a) => write!(w, "hsla({h}, {s}%, {l}%, {a})"),
            Self::Named(name) => w.write_str(name),
        }
    }
}

/// A CSS value that can be written directly to a style string buffer.
///
/// This is the unified type for all CSS values. It provides typed representations of CSS
/// values that avoid heap allocation when writing to the style buffer.
///
/// # Ergonomics
///
/// Use the free convenience functions for common values:
/// ```rust
/// use leptos_styles::css::{px, em, rem, pct, deg};
///
/// let width = px(100);       // CssDimension::Length(CssLength::Px(100.0))
/// let margin = em(0.6);      // CssDimension::Length(CssLength::Em(0.6))
/// let offset = pct(50);      // CssDimension::Percent(50.0)
/// let rotation = deg(45);    // CssValue::Angle(CssAngle::Deg(45.0))
/// ```
///
/// # String fallback
///
/// For complex or uncommon CSS values, use `Str`:
/// ```rust
/// use leptos_styles::css::CssValue;
///
/// let gradient = CssValue::Str("linear-gradient(to right, red, blue)".into());
/// let transform = CssValue::Str("translate(-50%, -50%)".into());
/// ```
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssValue {
    /// An arbitrary string value (static, owned, or reference-counted).
    ///
    /// Use this for CSS values that don't have a dedicated variant (e.g., gradients, transforms).
    Str(Oco<'static, str>),
    /// A bare floating-point number (e.g., opacity, flex-grow).
    Number(f64),
    /// A bare integer (e.g., z-index, order).
    Integer(i32),
    /// A CSS length value (px, em, rem, vh, vw, etc.).
    Length(CssLength),
    /// A percentage value.
    Percent(f64),
    /// A CSS angle value (deg, rad, turn, grad).
    Angle(CssAngle),
    /// A CSS time value (s, ms).
    Time(CssTime),
    /// A CSS color value (rgb, rgba, hsl, hsla, named).
    Color(CssColor),
    /// Fractional units for CSS Grid (e.g., `1fr`).
    Fr(f64),
    /// The CSS `auto` keyword.
    Auto,
    /// Zero with unit for `calc()` compatibility. Renders as `"0px"`.
    Zero,
    /// The CSS `inherit` keyword.
    Inherit,
    /// The CSS `initial` keyword.
    Initial,
    /// The CSS `unset` keyword.
    Unset,
    /// The CSS `revert` keyword.
    Revert,
    /// A CSS `var()` custom property reference.
    ///
    /// The name is stored **without** the `--` prefix. An optional fallback value can be provided.
    Var(Oco<'static, str>, Option<Box<CssValue>>),
    /// A CSS `calc()` expression.
    ///
    /// The expression is stored **without** the `calc()` wrapper.
    Calc(Oco<'static, str>),
    /// A CSS `min()` expression. Contents stored **without** the `min()` wrapper.
    Min(Oco<'static, str>),
    /// A CSS `max()` expression. Contents stored **without** the `max()` wrapper.
    Max(Oco<'static, str>),
    /// A CSS `clamp()` expression. Contents stored **without** the `clamp()` wrapper.
    Clamp(Oco<'static, str>),
    /// A CSS `env()` expression. Contents stored **without** the `env()` wrapper.
    Env(Oco<'static, str>),
}

impl CssWriteTo for CssValue {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Str(s) => w.write_str(s),
            Self::Number(v) => write!(w, "{v}"),
            Self::Integer(v) => write!(w, "{v}"),
            Self::Length(l) => l.css_fmt(w),
            Self::Percent(v) => write!(w, "{v}%"),
            Self::Angle(a) => a.css_fmt(w),
            Self::Time(t) => t.css_fmt(w),
            Self::Color(c) => c.css_fmt(w),
            Self::Fr(v) => write!(w, "{v}fr"),
            Self::Auto => w.write_str("auto"),
            Self::Zero => w.write_str("0px"),
            Self::Inherit => w.write_str("inherit"),
            Self::Initial => w.write_str("initial"),
            Self::Unset => w.write_str("unset"),
            Self::Revert => w.write_str("revert"),
            Self::Var(name, None) => write!(w, "var(--{name})"),
            Self::Var(name, Some(fallback)) => {
                write!(w, "var(--{name}, ")?;
                fallback.css_fmt(w)?;
                w.write_char(')')
            }
            Self::Calc(expr) => write!(w, "calc({expr})"),
            Self::Min(expr) => write!(w, "min({expr})"),
            Self::Max(expr) => write!(w, "max({expr})"),
            Self::Clamp(expr) => write!(w, "clamp({expr})"),
            Self::Env(expr) => write!(w, "env({expr})"),
        }
    }
}

// --- From impls for CssValue ---

impl From<&'static str> for CssValue {
    fn from(s: &'static str) -> Self {
        Self::Str(Oco::Borrowed(s))
    }
}

impl From<String> for CssValue {
    fn from(s: String) -> Self {
        Self::Str(Oco::Owned(s))
    }
}

impl From<Oco<'static, str>> for CssValue {
    fn from(s: Oco<'static, str>) -> Self {
        Self::Str(s)
    }
}

impl From<std::borrow::Cow<'static, str>> for CssValue {
    fn from(cow: std::borrow::Cow<'static, str>) -> Self {
        match cow {
            std::borrow::Cow::Borrowed(s) => Self::Str(Oco::Borrowed(s)),
            std::borrow::Cow::Owned(s) => Self::Str(Oco::Owned(s)),
        }
    }
}

impl From<CssLength> for CssValue {
    fn from(l: CssLength) -> Self {
        Self::Length(l)
    }
}

impl From<CssAngle> for CssValue {
    fn from(a: CssAngle) -> Self {
        Self::Angle(a)
    }
}

impl From<CssTime> for CssValue {
    fn from(t: CssTime) -> Self {
        Self::Time(t)
    }
}

impl From<CssColor> for CssValue {
    fn from(c: CssColor) -> Self {
        Self::Color(c)
    }
}

impl From<i32> for CssValue {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl From<f64> for CssValue {
    fn from(v: f64) -> Self {
        Self::Number(v)
    }
}

/// A CSS dimension value: length, percentage, `auto`, or `zero`.
///
/// This is a type-safe subset of [`CssValue`] for component props that specifically need
/// sizing values (width, height, gap, spacing, margin, etc.). It prevents accidentally
/// passing angles, times, or arbitrary strings where a dimension is expected.
///
/// Converts to [`CssValue`] via `From`/`Into` for use with the style system.
///
/// # Example
/// ```rust
/// use leptos_styles::css::{CssDimension, CssLength};
///
/// let gap = CssDimension::Length(CssLength::Em(0.6));
/// let width = CssDimension::Percent(100.0);
/// let height = CssDimension::Auto;
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssDimension {
    /// A CSS length value (px, em, rem, vh, vw, etc.).
    Length(CssLength),
    /// A percentage value.
    Percent(f64),
    /// The CSS `auto` keyword.
    Auto,
    /// Zero with unit for `calc()` compatibility. Renders as `"0px"`.
    Zero,
}

impl CssDimension {
    pub const fn px(v: f64) -> Self {
        Self::Length(CssLength::Px(v))
    }
    pub const fn em(v: f64) -> Self {
        Self::Length(CssLength::Em(v))
    }
    pub const fn rem(v: f64) -> Self {
        Self::Length(CssLength::Rem(v))
    }
    pub const fn vw(v: f64) -> Self {
        Self::Length(CssLength::Vw(v))
    }
    pub const fn vh(v: f64) -> Self {
        Self::Length(CssLength::Vh(v))
    }
    pub const fn vmin(v: f64) -> Self {
        Self::Length(CssLength::Vmin(v))
    }
    pub const fn vmax(v: f64) -> Self {
        Self::Length(CssLength::Vmax(v))
    }
    pub const fn ch(v: f64) -> Self {
        Self::Length(CssLength::Ch(v))
    }
    pub const fn dvw(v: f64) -> Self {
        Self::Length(CssLength::Dvw(v))
    }
    pub const fn dvh(v: f64) -> Self {
        Self::Length(CssLength::Dvh(v))
    }
    pub const fn svw(v: f64) -> Self {
        Self::Length(CssLength::Svw(v))
    }
    pub const fn svh(v: f64) -> Self {
        Self::Length(CssLength::Svh(v))
    }
    pub const fn lvw(v: f64) -> Self {
        Self::Length(CssLength::Lvw(v))
    }
    pub const fn lvh(v: f64) -> Self {
        Self::Length(CssLength::Lvh(v))
    }
    pub const fn cqw(v: f64) -> Self {
        Self::Length(CssLength::Cqw(v))
    }
    pub const fn cqh(v: f64) -> Self {
        Self::Length(CssLength::Cqh(v))
    }
    pub const fn pct(v: f64) -> Self {
        Self::Percent(v)
    }
}

impl CssWriteTo for CssDimension {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Length(l) => l.css_fmt(w),
            Self::Percent(v) => write!(w, "{v}%"),
            Self::Auto => w.write_str("auto"),
            Self::Zero => w.write_str("0px"),
        }
    }
}

impl From<CssDimension> for CssValue {
    fn from(dim: CssDimension) -> Self {
        match dim {
            CssDimension::Length(l) => Self::Length(l),
            CssDimension::Percent(v) => Self::Percent(v),
            CssDimension::Auto => Self::Auto,
            CssDimension::Zero => Self::Zero,
        }
    }
}

impl From<CssLength> for CssDimension {
    fn from(l: CssLength) -> Self {
        Self::Length(l)
    }
}

/// A CSS dimension value that may include function expressions.
///
/// This is the non-`Copy` superset of [`CssDimension`]. Use `CssDimension` when you need
/// `Copy` semantics (signal storage, const contexts). Use `CssDimensionExpr` when you need
/// to represent `calc()`, `min()`, `max()`, `clamp()`, or `env()` expressions.
///
/// All simple `CssDimension` values can be converted into `CssDimensionExpr` via [`From`].
///
/// # Example
/// ```rust
/// use leptos_styles::css::{CssDimensionExpr, px, pct};
///
/// let simple = CssDimensionExpr::from(px(100));
/// let calc = CssDimensionExpr::Calc("100% - 20px".into());
/// let clamped = CssDimensionExpr::Clamp("200px, 50%, 800px".into());
/// ```
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CssDimensionExpr {
    /// A simple dimension value (the [`Copy`]-compatible subset).
    Simple(CssDimension),
    /// A CSS `calc()` expression for arithmetic on mixed units.
    /// Contents stored **without** the `calc()` wrapper (e.g. `"100% - 20px"`).
    Calc(Oco<'static, str>),
    /// A CSS `min()` expression that resolves to the smallest of its arguments.
    /// Contents stored **without** the `min()` wrapper (e.g. `"50vw, 300px"`).
    Min(Oco<'static, str>),
    /// A CSS `max()` expression that resolves to the largest of its arguments.
    /// Contents stored **without** the `max()` wrapper (e.g. `"200px, 50%"`).
    Max(Oco<'static, str>),
    /// A CSS `clamp(min, preferred, max)` expression.
    /// Contents stored **without** the `clamp()` wrapper (e.g. `"200px, 50%, 800px"`).
    Clamp(Oco<'static, str>),
    /// A CSS `env()` expression for user-agent-defined environment variables.
    /// Common values: `safe-area-inset-top`, `safe-area-inset-bottom`, etc.
    /// Contents stored **without** the `env()` wrapper.
    Env(Oco<'static, str>),
}

impl CssWriteTo for CssDimensionExpr {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Simple(dim) => dim.css_fmt(w),
            Self::Calc(expr) => write!(w, "calc({expr})"),
            Self::Min(expr) => write!(w, "min({expr})"),
            Self::Max(expr) => write!(w, "max({expr})"),
            Self::Clamp(expr) => write!(w, "clamp({expr})"),
            Self::Env(expr) => write!(w, "env({expr})"),
        }
    }
}

impl From<CssDimension> for CssDimensionExpr {
    fn from(dim: CssDimension) -> Self {
        Self::Simple(dim)
    }
}

impl From<CssLength> for CssDimensionExpr {
    fn from(l: CssLength) -> Self {
        Self::Simple(CssDimension::Length(l))
    }
}

impl From<CssDimensionExpr> for CssValue {
    fn from(expr: CssDimensionExpr) -> Self {
        match expr {
            CssDimensionExpr::Simple(dim) => dim.into(),
            CssDimensionExpr::Calc(e) => Self::Calc(e),
            CssDimensionExpr::Min(e) => Self::Min(e),
            CssDimensionExpr::Max(e) => Self::Max(e),
            CssDimensionExpr::Clamp(e) => Self::Clamp(e),
            CssDimensionExpr::Env(e) => Self::Env(e),
        }
    }
}

// --- CSS shorthand value types ---

/// CSS font-weight values.
///
/// Represents the `font-weight` CSS property as a typed enum rather than a raw string.
/// Numeric weights range from 100 (thinnest) to 900 (boldest), with keyword aliases
/// for common values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    /// Weight 100 — Thin / Hairline.
    W100,
    /// Weight 200 — Extra Light / Ultra Light.
    W200,
    /// Weight 300 — Light.
    W300,
    /// Weight 400 — Normal / Regular (same as `WNormal`).
    W400,
    /// Weight 500 — Medium.
    W500,
    /// Weight 600 — Semi Bold / Demi Bold.
    W600,
    /// Weight 700 — Bold (same as `WBold`).
    W700,
    /// Weight 800 — Extra Bold / Ultra Bold.
    W800,
    /// Weight 900 — Black / Heavy.
    W900,
    /// Keyword `lighter` — one relative step lighter than the inherited weight.
    WLighter,
    /// Keyword `normal` — equivalent to weight 400.
    WNormal,
    /// Keyword `bold` — equivalent to weight 700.
    WBold,
    /// Keyword `bolder` — one relative step bolder than the inherited weight.
    WBolder,
}

impl CssWriteTo for FontWeight {
    fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::W100 => w.write_str("100"),
            Self::W200 => w.write_str("200"),
            Self::W300 => w.write_str("300"),
            Self::W400 => w.write_str("400"),
            Self::W500 => w.write_str("500"),
            Self::W600 => w.write_str("600"),
            Self::W700 => w.write_str("700"),
            Self::W800 => w.write_str("800"),
            Self::W900 => w.write_str("900"),
            Self::WLighter => w.write_str("lighter"),
            Self::WNormal => w.write_str("normal"),
            Self::WBold => w.write_str("bold"),
            Self::WBolder => w.write_str("bolder"),
        }
    }
}

impl From<FontWeight> for CssValue {
    fn from(fw: FontWeight) -> Self {
        match fw {
            FontWeight::W100 => Self::Integer(100),
            FontWeight::W200 => Self::Integer(200),
            FontWeight::W300 => Self::Integer(300),
            FontWeight::W400 => Self::Integer(400),
            FontWeight::W500 => Self::Integer(500),
            FontWeight::W600 => Self::Integer(600),
            FontWeight::W700 => Self::Integer(700),
            FontWeight::W800 => Self::Integer(800),
            FontWeight::W900 => Self::Integer(900),
            FontWeight::WLighter => Self::Str(Oco::Borrowed("lighter")),
            FontWeight::WNormal => Self::Str(Oco::Borrowed("normal")),
            FontWeight::WBold => Self::Str(Oco::Borrowed("bold")),
            FontWeight::WBolder => Self::Str(Oco::Borrowed("bolder")),
        }
    }
}

macro_rules! box_model_shorthand {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $name {
            /// Only on the top edge (other edges are `0`).
            Top(CssDimension),
            /// Only on the right edge (other edges are `0`).
            Right(CssDimension),
            /// Only on the bottom edge (other edges are `0`).
            Bottom(CssDimension),
            /// Only on the left edge (other edges are `0`).
            Left(CssDimension),
            /// Equal on all four edges.
            All(CssDimension),
            /// Separate vertical (top/bottom) and horizontal (left/right).
            Double(CssDimension, CssDimension),
            /// Explicit for each edge: top, right, bottom, left.
            Full(CssDimension, CssDimension, CssDimension, CssDimension),
        }

        impl CssWriteTo for $name {
            fn css_fmt<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                match self {
                    Self::Top(size) => {
                        size.css_fmt(w)?;
                        w.write_str(" 0 0 0")
                    }
                    Self::Right(size) => {
                        w.write_str("0 ")?;
                        size.css_fmt(w)?;
                        w.write_str(" 0 0")
                    }
                    Self::Bottom(size) => {
                        w.write_str("0 0 ")?;
                        size.css_fmt(w)?;
                        w.write_str(" 0")
                    }
                    Self::Left(size) => {
                        w.write_str("0 0 0 ")?;
                        size.css_fmt(w)
                    }
                    Self::All(size) => size.css_fmt(w),
                    Self::Double(vertical, horizontal) => {
                        vertical.css_fmt(w)?;
                        w.write_char(' ')?;
                        horizontal.css_fmt(w)
                    }
                    Self::Full(top, right, bottom, left) => {
                        top.css_fmt(w)?;
                        w.write_char(' ')?;
                        right.css_fmt(w)?;
                        w.write_char(' ')?;
                        bottom.css_fmt(w)?;
                        w.write_char(' ')?;
                        left.css_fmt(w)
                    }
                }
            }
        }

        impl From<$name> for CssValue {
            fn from(v: $name) -> Self {
                let mut buf = String::new();
                v.write_to(&mut buf);
                Self::Str(buf.into())
            }
        }
    };
}

box_model_shorthand!(
    /// CSS margin shorthand values.
    ///
    /// Represents the `margin` CSS shorthand as a typed enum. Each variant maps to
    /// a different shorthand form:
    ///
    /// - `All(dim)` → `margin: dim`
    /// - `Double(v, h)` → `margin: v h`
    /// - `Full(t, r, b, l)` → `margin: t r b l`
    /// - `Top(dim)` → `margin: dim 0 0 0` (and similarly for other sides)
    Margin
);

box_model_shorthand!(
    /// CSS padding shorthand values.
    ///
    /// Mirrors the [`Margin`] type but for the `padding` property.
    ///
    /// - `All(dim)` → `padding: dim`
    /// - `Double(v, h)` → `padding: v h`
    /// - `Full(t, r, b, l)` → `padding: t r b l`
    /// - `Top(dim)` → `padding: dim 0 0 0` (and similarly for other sides)
    Padding
);

impl_display_via_css_fmt!(
    CssLength,
    CssAngle,
    CssTime,
    CssColor,
    CssValue,
    CssDimension,
    CssDimensionExpr,
    FontWeight,
    Margin,
    Padding,
);

// --- Neg impls ---

impl std::ops::Neg for CssLength {
    type Output = Self;

    fn neg(self) -> Self {
        self.map_value(|v| -v)
    }
}

impl std::ops::Neg for CssDimension {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Length(l) => Self::Length(-l),
            Self::Percent(v) => Self::Percent(-v),
            Self::Zero => Self::Zero,
            Self::Auto => panic_auto_dimension_arithmetic("negation"),
        }
    }
}

impl std::ops::Neg for CssAngle {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Deg(v) => Self::Deg(-v),
            Self::Rad(v) => Self::Rad(-v),
            Self::Turn(v) => Self::Turn(-v),
            Self::Grad(v) => Self::Grad(-v),
        }
    }
}

impl std::ops::Neg for CssTime {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::S(v) => Self::S(-v),
            Self::Ms(v) => Self::Ms(-v),
        }
    }
}

// --- Dimension arithmetic ---

/// Returns true if two CssLength values use the same unit (ignoring the numeric value).
fn same_unit(a: &CssLength, b: &CssLength) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn is_auto_dimension(dim: &CssDimension) -> bool {
    matches!(dim, CssDimension::Auto)
}

fn assert_no_auto_dimension(dim: &CssDimension, operation: &str) {
    assert!(
        !is_auto_dimension(dim),
        "CssDimension::Auto cannot be used in CSS dimension arithmetic ({operation})"
    );
}

fn assert_no_auto_dimension_expr(expr: &CssDimensionExpr, operation: &str) {
    if let CssDimensionExpr::Simple(dim) = expr {
        assert_no_auto_dimension(dim, operation);
    }
}

fn panic_auto_dimension_arithmetic(operation: &str) -> ! {
    panic!("CssDimension::Auto cannot be used in CSS dimension arithmetic ({operation})")
}

impl std::ops::Add for CssDimension {
    type Output = CssDimensionExpr;

    /// Add two dimension values. Same-unit operations fold (e.g. `px(10) + px(5)` = `px(15)`).
    /// Mixed-unit operations produce a `calc()` expression.
    fn add(self, rhs: Self) -> CssDimensionExpr {
        match (self, rhs) {
            (Self::Auto, _) | (_, Self::Auto) => panic_auto_dimension_arithmetic("addition"),
            (Self::Zero, other) | (other, Self::Zero) => CssDimensionExpr::Simple(other),
            (Self::Length(a), Self::Length(b)) if same_unit(&a, &b) => {
                CssDimensionExpr::Simple(Self::Length(a.map_value(|v| v + b.value())))
            }
            (Self::Percent(a), Self::Percent(b)) => CssDimensionExpr::Simple(Self::Percent(a + b)),
            (a, b) => CssDimensionExpr::Calc(format!("{a} + {b}").into()),
        }
    }
}

impl std::ops::Sub for CssDimension {
    type Output = CssDimensionExpr;

    /// Subtract two dimension values. Same-unit operations fold (e.g. `px(10) - px(5)` = `px(5)`).
    /// Mixed-unit operations produce a `calc()` expression.
    fn sub(self, rhs: Self) -> CssDimensionExpr {
        match (self, rhs) {
            (Self::Auto, _) | (_, Self::Auto) => panic_auto_dimension_arithmetic("subtraction"),
            (lhs, Self::Zero) => CssDimensionExpr::Simple(lhs),
            (Self::Zero, rhs) => CssDimensionExpr::Simple(-rhs),
            (Self::Length(a), Self::Length(b)) if same_unit(&a, &b) => {
                CssDimensionExpr::Simple(Self::Length(a.map_value(|v| v - b.value())))
            }
            (Self::Percent(a), Self::Percent(b)) => CssDimensionExpr::Simple(Self::Percent(a - b)),
            (a, b) => CssDimensionExpr::Calc(format!("{a} - {b}").into()),
        }
    }
}

impl std::ops::Add<CssDimension> for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn add(self, rhs: CssDimension) -> CssDimensionExpr {
        match self {
            Self::Simple(lhs) => lhs + rhs,
            other => {
                assert_no_auto_dimension(&rhs, "addition");
                let mut buf = String::new();
                other.write_to(&mut buf);
                CssDimensionExpr::Calc(format!("{buf} + {rhs}").into())
            }
        }
    }
}

impl std::ops::Sub<CssDimension> for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn sub(self, rhs: CssDimension) -> CssDimensionExpr {
        match self {
            Self::Simple(lhs) => lhs - rhs,
            other => {
                assert_no_auto_dimension(&rhs, "subtraction");
                let mut buf = String::new();
                other.write_to(&mut buf);
                CssDimensionExpr::Calc(format!("{buf} - {rhs}").into())
            }
        }
    }
}

impl std::ops::Add for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Simple(a), Self::Simple(b)) => a + b,
            (lhs, rhs) => {
                assert_no_auto_dimension_expr(&lhs, "addition");
                assert_no_auto_dimension_expr(&rhs, "addition");
                let mut buf_l = String::new();
                lhs.write_to(&mut buf_l);
                let mut buf_r = String::new();
                rhs.write_to(&mut buf_r);
                CssDimensionExpr::Calc(format!("{buf_l} + {buf_r}").into())
            }
        }
    }
}

impl std::ops::Sub for CssDimensionExpr {
    type Output = CssDimensionExpr;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Simple(a), Self::Simple(b)) => a - b,
            (lhs, rhs) => {
                assert_no_auto_dimension_expr(&lhs, "subtraction");
                assert_no_auto_dimension_expr(&rhs, "subtraction");
                let mut buf_l = String::new();
                lhs.write_to(&mut buf_l);
                let mut buf_r = String::new();
                rhs.write_to(&mut buf_r);
                CssDimensionExpr::Calc(format!("{buf_l} - {buf_r}").into())
            }
        }
    }
}

// --- Free convenience functions ---

// Length and percentage functions return CssDimension for type-safe use in dimension props.
// They also work in style closures via Into<CssValue>.

/// Asserts in debug builds that a CSS numeric value is finite (not NaN or Infinity).
#[inline]
fn debug_assert_finite(v: f64, unit: &str) {
    debug_assert!(v.is_finite(), "CSS {unit} value must be finite, got {v}");
}

#[inline]
fn debug_assert_in_range(v: f64, label: &str, min: f64, max: f64) {
    debug_assert!(
        v.is_finite() && (min..=max).contains(&v),
        "CSS {label} must be in range [{min}, {max}], got {v}"
    );
}

#[inline]
fn debug_assert_non_negative(v: f64, label: &str) {
    debug_assert!(
        v.is_finite() && v >= 0.0,
        "CSS {label} must be non-negative, got {v}"
    );
}

macro_rules! dim_fn {
    ($(#[$meta:meta])* $fn_name:ident, $variant:ident, $unit_str:literal) => {
        $(#[$meta])*
        pub fn $fn_name(v: impl Into<f64>) -> CssDimension {
            let v = v.into();
            debug_assert_finite(v, $unit_str);
            CssDimension::Length(CssLength::$variant(v))
        }
    };
}

dim_fn!(/// Create a CSS pixel value.
    px, Px, "px");
dim_fn!(/// Create a CSS em value (relative to the element's font size).
    em, Em, "em");
dim_fn!(
    #[allow(clippy::module_name_repetitions)]
    /// Create a CSS rem value (relative to the root element's font size).
    rem,
    Rem,
    "rem"
);
dim_fn!(/// Create a CSS viewport-width value (1vw = 1% of the viewport width).
    vw, Vw, "vw");
dim_fn!(/// Create a CSS viewport-height value (1vh = 1% of the viewport height).
    vh, Vh, "vh");
dim_fn!(/// Create a CSS vmin value (1vmin = 1% of the smaller viewport dimension).
    vmin, Vmin, "vmin");
dim_fn!(/// Create a CSS vmax value (1vmax = 1% of the larger viewport dimension).
    vmax, Vmax, "vmax");
dim_fn!(/// Create a CSS ch (character width) value — the width of the `0` glyph in the element's font.
    ch, Ch, "ch");
dim_fn!(/// Create a CSS dynamic viewport width value.
    /// Unlike `vw`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    dvw, Dvw, "dvw");
dim_fn!(/// Create a CSS dynamic viewport height value.
    /// Unlike `vh`, adjusts when browser UI elements (e.g. mobile address bar) appear or disappear.
    dvh, Dvh, "dvh");
dim_fn!(/// Create a CSS small viewport width value.
    /// Represents the viewport width when all dynamic browser UI is expanded/visible.
    svw, Svw, "svw");
dim_fn!(/// Create a CSS small viewport height value.
    /// Represents the viewport height when all dynamic browser UI is expanded/visible.
    svh, Svh, "svh");
dim_fn!(/// Create a CSS large viewport width value.
    /// Represents the viewport width when all dynamic browser UI is retracted/hidden.
    lvw, Lvw, "lvw");
dim_fn!(/// Create a CSS large viewport height value.
    /// Represents the viewport height when all dynamic browser UI is retracted/hidden.
    lvh, Lvh, "lvh");
dim_fn!(/// Create a CSS container query width value (1cqw = 1% of the nearest size container's width).
    cqw, Cqw, "cqw");
dim_fn!(/// Create a CSS container query height value (1cqh = 1% of the nearest size container's height).
    cqh, Cqh, "cqh");

/// Create a CSS percentage value.
pub fn pct(v: impl Into<f64>) -> CssDimension {
    let v = v.into();
    debug_assert_finite(v, "%");
    CssDimension::Percent(v)
}

/// Create a CSS degree angle value.
pub fn deg(v: impl Into<f64>) -> CssValue {
    let v = v.into();
    debug_assert_finite(v, "deg");
    CssValue::Angle(CssAngle::Deg(v))
}

/// Create a CSS radian angle value.
pub fn rad(v: impl Into<f64>) -> CssValue {
    let v = v.into();
    debug_assert_finite(v, "rad");
    CssValue::Angle(CssAngle::Rad(v))
}

/// Create a CSS turn angle value.
pub fn turn(v: impl Into<f64>) -> CssValue {
    let v = v.into();
    debug_assert_finite(v, "turn");
    CssValue::Angle(CssAngle::Turn(v))
}

/// Create a CSS gradian angle value.
pub fn grad(v: impl Into<f64>) -> CssValue {
    let v = v.into();
    debug_assert_finite(v, "grad");
    CssValue::Angle(CssAngle::Grad(v))
}

/// Create a CSS seconds time value.
pub fn s(v: impl Into<f64>) -> CssValue {
    let v = v.into();
    debug_assert_finite(v, "s");
    CssValue::Time(CssTime::S(v))
}

/// Create a CSS milliseconds time value.
pub fn ms(v: impl Into<f64>) -> CssValue {
    let v = v.into();
    debug_assert_finite(v, "ms");
    CssValue::Time(CssTime::Ms(v))
}

/// Create a CSS fractional unit value (for CSS Grid).
pub fn fr(v: impl Into<f64>) -> CssValue {
    let v = v.into();
    debug_assert_non_negative(v, "fr");
    CssValue::Fr(v)
}

/// Create a CSS RGB color value.
pub fn rgb(r: u8, g: u8, b: u8) -> CssValue {
    CssValue::Color(CssColor::Rgb(r, g, b))
}

/// Create a CSS RGBA color value with alpha.
pub fn rgba(r: u8, g: u8, b: u8, a: f64) -> CssValue {
    debug_assert_in_range(a, "rgba alpha", 0.0, 1.0);
    CssValue::Color(CssColor::Rgba(r, g, b, a))
}

/// Create a CSS HSL color value.
///
/// * `h` - Hue in degrees (0-360).
/// * `s` - Saturation as a percentage (0-100).
/// * `l` - Lightness as a percentage (0-100).
pub fn hsl(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>) -> CssValue {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    debug_assert_finite(h, "hsl hue");
    debug_assert_in_range(s, "hsl saturation", 0.0, 100.0);
    debug_assert_in_range(l, "hsl lightness", 0.0, 100.0);
    CssValue::Color(CssColor::Hsl(h, s, l))
}

/// Create a CSS HSLA color value with alpha.
///
/// * `h` - Hue in degrees (0-360).
/// * `s` - Saturation as a percentage (0-100).
/// * `l` - Lightness as a percentage (0-100).
/// * `a` - Alpha (0.0-1.0).
pub fn hsla(
    h: impl Into<f64>,
    s: impl Into<f64>,
    l: impl Into<f64>,
    a: impl Into<f64>,
) -> CssValue {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    let a = a.into();
    debug_assert_finite(h, "hsla hue");
    debug_assert_in_range(s, "hsla saturation", 0.0, 100.0);
    debug_assert_in_range(l, "hsla lightness", 0.0, 100.0);
    debug_assert_in_range(a, "hsla alpha", 0.0, 1.0);
    CssValue::Color(CssColor::Hsla(h, s, l, a))
}

fn normalize_custom_property_name(name: Oco<'static, str>) -> Oco<'static, str> {
    let normalized = {
        let original: &str = &name;
        let trimmed = original.trim();
        let stripped = trimmed.strip_prefix("--").unwrap_or(trimmed);
        assert!(
            !stripped.is_empty(),
            "CSS custom property name must not be empty"
        );

        if stripped == original {
            None
        } else {
            Some(stripped.to_string())
        }
    };

    match normalized {
        Some(name) => Oco::Owned(name),
        None => name,
    }
}

/// Create a CSS `var()` custom property reference.
///
/// The `name` can be passed with or without the `--` prefix.
/// ```rust
/// use leptos_styles::css::var;
///
/// let v = var("spacing");  // renders as: var(--spacing)
/// let v = var("--spacing"); // also renders as: var(--spacing)
/// ```
pub fn var(name: impl Into<Oco<'static, str>>) -> CssValue {
    CssValue::Var(normalize_custom_property_name(name.into()), None)
}

/// Create a CSS `var()` custom property reference with a fallback value.
///
/// The `name` can be passed with or without the `--` prefix.
/// ```rust
/// use leptos_styles::css::{var_with_fallback, px};
///
/// let v = var_with_fallback("spacing", px(8));  // renders as: var(--spacing, 8px)
/// let v = var_with_fallback("--spacing", px(8)); // also renders as: var(--spacing, 8px)
/// ```
pub fn var_with_fallback(
    name: impl Into<Oco<'static, str>>,
    fallback: impl Into<CssValue>,
) -> CssValue {
    CssValue::Var(
        normalize_custom_property_name(name.into()),
        Some(Box::new(fallback.into())),
    )
}

/// Create a CSS `calc()` expression.
///
/// The expression should be provided **without** the `calc()` wrapper.
/// ```rust
/// use leptos_styles::css::calc;
///
/// let v = calc("100% - 2rem");  // renders as: calc(100% - 2rem)
/// ```
pub fn calc(expr: impl Into<Oco<'static, str>>) -> CssValue {
    CssValue::Calc(expr.into())
}

/// Create a CSS `calc()` dimension expression for arithmetic on mixed units.
///
/// Unlike [`calc()`] which returns a general [`CssValue`], this returns a [`CssDimensionExpr`]
/// for use in dimension-typed contexts (width, height, gap, etc.).
///
/// The expression should be provided **without** the `calc()` wrapper.
/// ```rust
/// use leptos_styles::css::dim_calc;
///
/// let v = dim_calc("100% - 20px");  // renders as: calc(100% - 20px)
/// ```
pub fn dim_calc(expr: impl Into<Oco<'static, str>>) -> CssDimensionExpr {
    CssDimensionExpr::Calc(expr.into())
}

/// Create a CSS `min()` dimension expression that resolves to the smallest of its arguments.
///
/// The expression should be provided **without** the `min()` wrapper.
/// ```rust
/// use leptos_styles::css::css_min;
///
/// let v = css_min("50vw, 300px");  // renders as: min(50vw, 300px)
/// ```
pub fn css_min(expr: impl Into<Oco<'static, str>>) -> CssDimensionExpr {
    CssDimensionExpr::Min(expr.into())
}

/// Create a CSS `max()` dimension expression that resolves to the largest of its arguments.
///
/// The expression should be provided **without** the `max()` wrapper.
/// ```rust
/// use leptos_styles::css::css_max;
///
/// let v = css_max("200px, 50%");  // renders as: max(200px, 50%)
/// ```
pub fn css_max(expr: impl Into<Oco<'static, str>>) -> CssDimensionExpr {
    CssDimensionExpr::Max(expr.into())
}

/// Create a CSS `clamp(min, preferred, max)` dimension expression.
///
/// The expression should be provided **without** the `clamp()` wrapper.
/// ```rust
/// use leptos_styles::css::css_clamp;
///
/// let v = css_clamp("200px, 50%, 800px");  // renders as: clamp(200px, 50%, 800px)
/// ```
pub fn css_clamp(expr: impl Into<Oco<'static, str>>) -> CssDimensionExpr {
    CssDimensionExpr::Clamp(expr.into())
}

/// Create a CSS `env()` dimension expression for user-agent-defined environment variables.
///
/// Common values: `safe-area-inset-top`, `safe-area-inset-bottom`,
/// `safe-area-inset-left`, `safe-area-inset-right`.
///
/// The expression should be provided **without** the `env()` wrapper.
/// ```rust
/// use leptos_styles::css::css_env;
///
/// let v = css_env("safe-area-inset-top");  // renders as: env(safe-area-inset-top)
/// ```
pub fn css_env(expr: impl Into<Oco<'static, str>>) -> CssDimensionExpr {
    CssDimensionExpr::Env(expr.into())
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn test_css_length_write_to() {
        let cases = [
            (CssLength::Px(10.0), "10px"),
            (CssLength::Px(0.0), "0px"),
            (CssLength::Em(1.5), "1.5em"),
            (CssLength::Rem(2.0), "2rem"),
            (CssLength::Vw(100.0), "100vw"),
            (CssLength::Vh(50.0), "50vh"),
            (CssLength::Vmin(25.0), "25vmin"),
            (CssLength::Vmax(75.0), "75vmax"),
            (CssLength::Ch(3.0), "3ch"),
            (CssLength::Dvw(50.0), "50dvw"),
            (CssLength::Dvh(100.0), "100dvh"),
            (CssLength::Svw(80.0), "80svw"),
            (CssLength::Svh(90.0), "90svh"),
            (CssLength::Lvw(100.0), "100lvw"),
            (CssLength::Lvh(100.0), "100lvh"),
            (CssLength::Cqw(50.0), "50cqw"),
            (CssLength::Cqh(25.0), "25cqh"),
        ];
        for (length, expected) in cases {
            let mut buf = String::new();
            length.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_angle_write_to() {
        let cases = [
            (CssAngle::Deg(45.0), "45deg"),
            (CssAngle::Rad(2.5), "2.5rad"),
            (CssAngle::Turn(0.5), "0.5turn"),
            (CssAngle::Grad(200.0), "200grad"),
        ];
        for (angle, expected) in cases {
            let mut buf = String::new();
            angle.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_time_write_to() {
        let cases = [(CssTime::S(0.3), "0.3s"), (CssTime::Ms(300.0), "300ms")];
        for (time, expected) in cases {
            let mut buf = String::new();
            time.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_value_write_to() {
        let cases: Vec<(CssValue, &str)> = vec![
            (CssValue::Str("flex".into()), "flex"),
            (CssValue::Str("computed".to_string().into()), "computed"),
            (CssValue::Number(0.5), "0.5"),
            (CssValue::Integer(10), "10"),
            (CssValue::Length(CssLength::Px(100.0)), "100px"),
            (CssValue::Percent(50.0), "50%"),
            (CssValue::Angle(CssAngle::Deg(90.0)), "90deg"),
            (CssValue::Time(CssTime::S(0.3)), "0.3s"),
            (CssValue::Fr(1.0), "1fr"),
            (CssValue::Auto, "auto"),
            (CssValue::Zero, "0px"),
            (CssValue::Inherit, "inherit"),
            (CssValue::Initial, "initial"),
            (CssValue::Unset, "unset"),
            (CssValue::Revert, "revert"),
        ];
        for (value, expected) in cases {
            let mut buf = String::new();
            value.write_to(&mut buf);
            assertr::assert_that!(buf).is_equal_to(expected.to_string());
        }
    }

    #[test]
    fn test_css_value_display() {
        assertr::assert_that!(px(10.0).to_string()).is_equal_to("10px".to_string());
        assertr::assert_that!(em(1.5).to_string()).is_equal_to("1.5em".to_string());
        assertr::assert_that!(pct(50.0).to_string()).is_equal_to("50%".to_string());
        assertr::assert_that!(deg(45.0).to_string()).is_equal_to("45deg".to_string());
        assertr::assert_that!(CssValue::Auto.to_string()).is_equal_to("auto".to_string());
        assertr::assert_that!(CssValue::Zero.to_string()).is_equal_to("0px".to_string());
    }

    #[test]
    fn test_convenience_functions() {
        assertr::assert_that!(format!("{}", px(100))).is_equal_to("100px".to_string());
        assertr::assert_that!(format!("{}", em(0.6))).is_equal_to("0.6em".to_string());
        assertr::assert_that!(format!("{}", rem(1.5))).is_equal_to("1.5rem".to_string());
        assertr::assert_that!(format!("{}", vw(100))).is_equal_to("100vw".to_string());
        assertr::assert_that!(format!("{}", vh(50))).is_equal_to("50vh".to_string());
        assertr::assert_that!(format!("{}", pct(75))).is_equal_to("75%".to_string());
        assertr::assert_that!(format!("{}", deg(180))).is_equal_to("180deg".to_string());
        assertr::assert_that!(format!("{}", s(0.3))).is_equal_to("0.3s".to_string());
        assertr::assert_that!(format!("{}", ms(300))).is_equal_to("300ms".to_string());
        assertr::assert_that!(format!("{}", fr(1))).is_equal_to("1fr".to_string());
    }

    #[test]
    fn test_modern_viewport_convenience_functions() {
        assertr::assert_that!(format!("{}", dvw(50))).is_equal_to("50dvw".to_string());
        assertr::assert_that!(format!("{}", dvh(100))).is_equal_to("100dvh".to_string());
        assertr::assert_that!(format!("{}", svw(80))).is_equal_to("80svw".to_string());
        assertr::assert_that!(format!("{}", svh(90))).is_equal_to("90svh".to_string());
        assertr::assert_that!(format!("{}", lvw(100))).is_equal_to("100lvw".to_string());
        assertr::assert_that!(format!("{}", lvh(100))).is_equal_to("100lvh".to_string());
        assertr::assert_that!(format!("{}", cqw(50))).is_equal_to("50cqw".to_string());
        assertr::assert_that!(format!("{}", cqh(25))).is_equal_to("25cqh".to_string());
    }

    #[test]
    fn test_css_dimension_modern_const_fns() {
        assertr::assert_that!(CssDimension::dvw(50.0).to_string()).is_equal_to("50dvw".to_string());
        assertr::assert_that!(CssDimension::dvh(100.0).to_string())
            .is_equal_to("100dvh".to_string());
        assertr::assert_that!(CssDimension::svw(80.0).to_string()).is_equal_to("80svw".to_string());
        assertr::assert_that!(CssDimension::svh(90.0).to_string()).is_equal_to("90svh".to_string());
        assertr::assert_that!(CssDimension::lvw(100.0).to_string())
            .is_equal_to("100lvw".to_string());
        assertr::assert_that!(CssDimension::lvh(100.0).to_string())
            .is_equal_to("100lvh".to_string());
        assertr::assert_that!(CssDimension::cqw(50.0).to_string()).is_equal_to("50cqw".to_string());
        assertr::assert_that!(CssDimension::cqh(25.0).to_string()).is_equal_to("25cqh".to_string());
    }

    #[test]
    fn test_from_static_str() {
        let v: CssValue = "red".into();
        assertr::assert_that!(format!("{v}")).is_equal_to("red".to_string());
    }

    #[test]
    fn test_from_string() {
        let v: CssValue = "computed".to_string().into();
        assertr::assert_that!(format!("{v}")).is_equal_to("computed".to_string());
    }

    #[test]
    fn test_from_oco() {
        let borrowed: CssValue = Oco::Borrowed("borrowed").into();
        assertr::assert_that!(format!("{borrowed}")).is_equal_to("borrowed".to_string());

        let owned: CssValue = Oco::<str>::Owned("owned".to_string()).into();
        assertr::assert_that!(format!("{owned}")).is_equal_to("owned".to_string());
    }

    #[test]
    fn test_from_cow() {
        use std::borrow::Cow;

        let borrowed: CssValue = Cow::Borrowed("borrowed").into();
        assertr::assert_that!(format!("{borrowed}")).is_equal_to("borrowed".to_string());

        let owned: CssValue = Cow::<str>::Owned("owned".to_string()).into();
        assertr::assert_that!(format!("{owned}")).is_equal_to("owned".to_string());
    }

    #[test]
    fn test_integer_accepts_i32() {
        let v = CssValue::Integer(42);
        assertr::assert_that!(format!("{v}")).is_equal_to("42".to_string());
    }

    #[test]
    fn test_px_accepts_i32() {
        let v = px(10_i32);
        assertr::assert_that!(format!("{v}")).is_equal_to("10px".to_string());
    }

    #[test]
    fn test_css_color_write_to() {
        let mut buf = String::new();
        CssColor::Rgb(255, 128, 0).write_to(&mut buf);
        assertr::assert_that!(buf).is_equal_to("rgb(255, 128, 0)".to_string());

        let mut buf = String::new();
        CssColor::Rgba(0, 0, 0, 0.5).write_to(&mut buf);
        assertr::assert_that!(buf).is_equal_to("rgba(0, 0, 0, 0.5)".to_string());
    }

    #[test]
    fn test_rgb_convenience() {
        assertr::assert_that!(rgb(255, 0, 0).to_string()).is_equal_to("rgb(255, 0, 0)".to_string());
        assertr::assert_that!(rgba(0, 0, 0, 0.5).to_string())
            .is_equal_to("rgba(0, 0, 0, 0.5)".to_string());
    }

    #[test]
    fn test_hsl_convenience() {
        assertr::assert_that!(hsl(120, 100, 50).to_string())
            .is_equal_to("hsl(120, 100%, 50%)".to_string());
        assertr::assert_that!(hsla(240, 50, 75, 0.8).to_string())
            .is_equal_to("hsla(240, 50%, 75%, 0.8)".to_string());
    }

    #[test]
    fn test_named_color() {
        let c = CssColor::Named("transparent");
        assertr::assert_that!(c.to_string()).is_equal_to("transparent".to_string());
    }

    #[test]
    fn test_from_i32() {
        let v: CssValue = 42_i32.into();
        assertr::assert_that!(v.to_string()).is_equal_to("42".to_string());
    }

    #[test]
    fn test_css_dimension_associated_fns() {
        assertr::assert_that!(CssDimension::em(1.5).to_string()).is_equal_to("1.5em".to_string());
        assertr::assert_that!(CssDimension::px(10.0).to_string()).is_equal_to("10px".to_string());
        assertr::assert_that!(CssDimension::pct(50.0).to_string()).is_equal_to("50%".to_string());
        assertr::assert_that!(CssDimension::rem(2.0).to_string()).is_equal_to("2rem".to_string());
    }

    #[test]
    fn test_css_dimension_const() {
        const DIM: CssDimension = CssDimension::em(3.5);
        assertr::assert_that!(DIM.to_string()).is_equal_to("3.5em".to_string());
    }

    #[test]
    fn test_css_dimension_is_copy() {
        let dim = CssDimension::px(10.0);
        let copy = dim;
        // Both are usable — dim was copied, not moved.
        assertr::assert_that!(dim.to_string()).is_equal_to(copy.to_string());
    }

    #[test]
    fn test_neg_css_length() {
        assertr::assert_that!((-CssLength::Px(10.0)).to_string()).is_equal_to("-10px".to_string());
        assertr::assert_that!((-CssLength::Em(1.5)).to_string()).is_equal_to("-1.5em".to_string());
    }

    #[test]
    fn test_neg_css_dimension() {
        assertr::assert_that!((-px(10)).to_string()).is_equal_to("-10px".to_string());
        assertr::assert_that!((-pct(50)).to_string()).is_equal_to("-50%".to_string());
        assertr::assert_that!((-CssDimension::Zero).to_string()).is_equal_to("0px".to_string());
    }

    #[test]
    #[should_panic(expected = "CssDimension::Auto cannot be used in CSS dimension arithmetic")]
    fn test_neg_css_dimension_auto_panics() {
        let _ = -CssDimension::Auto;
    }

    #[test]
    fn test_neg_css_angle() {
        assertr::assert_that!((-CssAngle::Deg(90.0)).to_string()).is_equal_to("-90deg".to_string());
    }

    #[test]
    fn test_neg_css_time() {
        assertr::assert_that!((-CssTime::Ms(300.0)).to_string()).is_equal_to("-300ms".to_string());
    }

    #[test]
    fn test_global_keywords() {
        assertr::assert_that!(CssValue::Inherit.to_string()).is_equal_to("inherit".to_string());
        assertr::assert_that!(CssValue::Initial.to_string()).is_equal_to("initial".to_string());
        assertr::assert_that!(CssValue::Unset.to_string()).is_equal_to("unset".to_string());
        assertr::assert_that!(CssValue::Revert.to_string()).is_equal_to("revert".to_string());
    }

    #[test]
    fn test_var() {
        assertr::assert_that!(var("spacing").to_string()).is_equal_to("var(--spacing)".to_string());
        assertr::assert_that!(var("--spacing").to_string())
            .is_equal_to("var(--spacing)".to_string());
        assertr::assert_that!(var(" --spacing ").to_string())
            .is_equal_to("var(--spacing)".to_string());
    }

    #[test]
    fn test_var_with_fallback() {
        assertr::assert_that!(var_with_fallback("spacing", px(8)).to_string())
            .is_equal_to("var(--spacing, 8px)".to_string());
        assertr::assert_that!(var_with_fallback("--spacing", px(8)).to_string())
            .is_equal_to("var(--spacing, 8px)".to_string());
    }

    #[test]
    #[should_panic(expected = "CSS custom property name must not be empty")]
    fn test_var_empty_name_panics() {
        let _ = var(" -- ");
    }

    #[test]
    fn test_calc() {
        assertr::assert_that!(calc("100% - 2rem").to_string())
            .is_equal_to("calc(100% - 2rem)".to_string());
    }

    #[test]
    #[should_panic(expected = "CSS px value must be finite")]
    fn test_px_nan_panics_in_debug() {
        let _ = px(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS % value must be finite")]
    fn test_pct_infinity_panics_in_debug() {
        let _ = pct(f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "CSS deg value must be finite")]
    fn test_deg_nan_panics_in_debug() {
        let _ = deg(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "CSS fr must be non-negative")]
    fn test_fr_negative_panics_in_debug() {
        let _ = fr(-1.0);
    }

    #[test]
    #[should_panic(expected = "CSS rgba alpha must be in range [0, 1]")]
    fn test_rgba_alpha_out_of_range_panics_in_debug() {
        let _ = rgba(0, 0, 0, 1.5);
    }

    #[test]
    #[should_panic(expected = "CSS hsl saturation must be in range [0, 100]")]
    fn test_hsl_saturation_out_of_range_panics_in_debug() {
        let _ = hsl(120.0, 120.0, 50.0);
    }

    #[test]
    #[should_panic(expected = "CSS hsla alpha must be in range [0, 1]")]
    fn test_hsla_alpha_out_of_range_panics_in_debug() {
        let _ = hsla(240.0, 50.0, 50.0, -0.1);
    }

    #[test]
    fn test_css_dimension_expr_simple() {
        let expr = CssDimensionExpr::from(px(100));
        assertr::assert_that!(expr.to_string()).is_equal_to("100px".to_string());
    }

    #[test]
    fn test_css_dimension_expr_calc() {
        assertr::assert_that!(dim_calc("100% - 20px").to_string())
            .is_equal_to("calc(100% - 20px)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_min() {
        assertr::assert_that!(css_min("50vw, 300px").to_string())
            .is_equal_to("min(50vw, 300px)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_max() {
        assertr::assert_that!(css_max("200px, 50%").to_string())
            .is_equal_to("max(200px, 50%)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_clamp() {
        assertr::assert_that!(css_clamp("200px, 50%, 800px").to_string())
            .is_equal_to("clamp(200px, 50%, 800px)".to_string());
    }

    #[test]
    fn test_css_dimension_expr_env() {
        assertr::assert_that!(css_env("safe-area-inset-top").to_string())
            .is_equal_to("env(safe-area-inset-top)".to_string());
    }

    #[test]
    fn test_dimension_add_same_unit_folds() {
        let result = px(10) + px(5);
        assertr::assert_that!(result.to_string()).is_equal_to("15px".to_string());
    }

    #[test]
    fn test_dimension_sub_same_unit_folds() {
        let result = px(10) - px(3);
        assertr::assert_that!(result.to_string()).is_equal_to("7px".to_string());
    }

    #[test]
    fn test_dimension_add_percent_folds() {
        let result = pct(60) + pct(40);
        assertr::assert_that!(result.to_string()).is_equal_to("100%".to_string());
    }

    #[test]
    fn test_dimension_add_mixed_produces_calc() {
        let result = pct(100) - px(20);
        assertr::assert_that!(result.to_string()).is_equal_to("calc(100% - 20px)".to_string());
    }

    #[test]
    fn test_dimension_add_zero_simplifies() {
        let result = px(10) + CssDimension::Zero;
        assertr::assert_that!(result.to_string()).is_equal_to("10px".to_string());
    }

    #[test]
    fn test_dimension_sub_zero_simplifies() {
        let result = pct(50) - CssDimension::Zero;
        assertr::assert_that!(result.to_string()).is_equal_to("50%".to_string());
    }

    #[test]
    #[should_panic(expected = "CssDimension::Auto cannot be used in CSS dimension arithmetic")]
    fn test_dimension_auto_plus_length_panics() {
        let _ = CssDimension::Auto + px(10);
    }

    #[test]
    #[should_panic(expected = "CssDimension::Auto cannot be used in CSS dimension arithmetic")]
    fn test_dimension_length_plus_auto_panics() {
        let _ = px(10) + CssDimension::Auto;
    }

    #[test]
    #[should_panic(expected = "CssDimension::Auto cannot be used in CSS dimension arithmetic")]
    fn test_dimension_auto_minus_length_panics() {
        let _ = CssDimension::Auto - px(10);
    }

    #[test]
    #[should_panic(expected = "CssDimension::Auto cannot be used in CSS dimension arithmetic")]
    fn test_dimension_zero_minus_auto_panics() {
        let _ = CssDimension::Zero - CssDimension::Auto;
    }

    #[test]
    #[should_panic(expected = "CssDimension::Auto cannot be used in CSS dimension arithmetic")]
    fn test_dimension_expr_add_auto_panics() {
        let _ = dim_calc("100% - 20px") + CssDimension::Auto;
    }

    #[test]
    #[should_panic(expected = "CssDimension::Auto cannot be used in CSS dimension arithmetic")]
    fn test_dimension_expr_simple_auto_add_expr_panics() {
        let _ = CssDimensionExpr::from(CssDimension::Auto) + dim_calc("100% - 20px");
    }

    #[test]
    fn test_dimension_expr_add_dimension() {
        let expr = dim_calc("100% - 20px") + px(10);
        assertr::assert_that!(expr.to_string())
            .is_equal_to("calc(calc(100% - 20px) + 10px)".to_string());
    }

    #[test]
    fn test_dimension_expr_add_expr() {
        let a = CssDimensionExpr::from(px(10));
        let b = CssDimensionExpr::from(px(5));
        let result = a + b;
        assertr::assert_that!(result.to_string()).is_equal_to("15px".to_string());
    }

    #[test]
    fn test_css_dimension_expr_into_css_value() {
        let v: CssValue = dim_calc("100% - 20px").into();
        assertr::assert_that!(v.to_string()).is_equal_to("calc(100% - 20px)".to_string());

        let v: CssValue = css_min("50vw, 300px").into();
        assertr::assert_that!(v.to_string()).is_equal_to("min(50vw, 300px)".to_string());

        let v: CssValue = CssDimensionExpr::from(px(10)).into();
        assertr::assert_that!(v.to_string()).is_equal_to("10px".to_string());
    }
}
