use std::{borrow::Cow, error::Error, fmt, str::FromStr};

use leptos::prelude::{Get, Signal};

/// An unchecked CSS property name used by the explicit raw-string APIs.
pub type UncheckedPropertyName = Cow<'static, str>;

/// An unchecked CSS value used by the explicit raw-string APIs.
pub type UncheckedStyleValue = Cow<'static, str>;

pub(crate) fn normalize_style_property(property: UncheckedPropertyName) -> UncheckedPropertyName {
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

/// Conversion into an unchecked CSS property name.
///
/// This trait is consumed only by APIs whose names end in `_unchecked`.
pub trait IntoUncheckedPropertyName {
    /// Convert `self` into an [`UncheckedPropertyName`].
    fn into_unchecked_property_name(self) -> UncheckedPropertyName;
}

impl IntoUncheckedPropertyName for &'static str {
    fn into_unchecked_property_name(self) -> UncheckedPropertyName {
        Cow::Borrowed(self)
    }
}

impl IntoUncheckedPropertyName for String {
    fn into_unchecked_property_name(self) -> UncheckedPropertyName {
        Cow::Owned(self)
    }
}

impl IntoUncheckedPropertyName for UncheckedPropertyName {
    fn into_unchecked_property_name(self) -> UncheckedPropertyName {
        self
    }
}

/// Error returned when parsing a raw CSS declaration into a [`StyleEntry`] fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseStyleEntryError {
    input: String,
    reason: &'static str,
}

impl ParseStyleEntryError {
    fn new(input: &str, reason: &'static str) -> Self {
        Self {
            input: input.to_string(),
            reason,
        }
    }

    /// The declaration string that failed to parse.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }

    /// A short reason describing why parsing failed.
    #[must_use]
    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

impl fmt::Display for ParseStyleEntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to parse CSS style declaration {:?}: {}",
            self.input, self.reason
        )
    }
}

impl Error for ParseStyleEntryError {}

fn parse_style_declaration(
    declaration: &str,
) -> Result<(UncheckedPropertyName, UncheckedStyleValue), ParseStyleEntryError> {
    let Some((property, value)) = declaration.split_once(':') else {
        return Err(ParseStyleEntryError::new(
            declaration,
            "missing ':' separator",
        ));
    };

    let property = property.trim();
    if property.is_empty() {
        return Err(ParseStyleEntryError::new(
            declaration,
            "missing property name",
        ));
    }

    let value = value.trim().trim_end_matches(';').trim();
    if value.is_empty() {
        return Err(ParseStyleEntryError::new(
            declaration,
            "missing property value",
        ));
    }

    Ok((
        Cow::Owned(property.to_string()),
        Cow::Owned(value.to_string()),
    ))
}

/// One resolved declaration, either from the checked boundary or an explicit unchecked API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StyleDeclaration {
    Unchecked {
        property: UncheckedPropertyName,
        value: UncheckedStyleValue,
    },
    #[cfg(feature = "typed-css")]
    Checked(leptos_css::CheckedDeclaration),
}

impl StyleDeclaration {
    pub(crate) fn unchecked(property: UncheckedPropertyName, value: UncheckedStyleValue) -> Self {
        Self::Unchecked {
            property: normalize_style_property(property),
            value,
        }
    }

    pub(crate) fn property_name(&self) -> &str {
        match self {
            Self::Unchecked { property, .. } => property,
            #[cfg(feature = "typed-css")]
            Self::Checked(declaration) => declaration.property_name(),
        }
    }

    pub(crate) fn write_to(&self, buf: &mut String) {
        match self {
            Self::Unchecked { property, value } => {
                buf.push_str(property);
                buf.push(':');
                buf.push_str(value);
                buf.push(';');
            }
            #[cfg(feature = "typed-css")]
            Self::Checked(declaration) => declaration.write_declaration_to(buf),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum StyleDeclarationSource {
    Static(Option<StyleDeclaration>),
    Reactive(Signal<Option<StyleDeclaration>>),
}

impl StyleDeclarationSource {
    fn resolve(&self) -> Option<Cow<'_, StyleDeclaration>> {
        match self {
            Self::Static(declaration) => declaration.as_ref().map(Cow::Borrowed),
            Self::Reactive(signal) => signal.get().map(Cow::Owned),
        }
    }

    fn static_declaration(&self) -> Option<&StyleDeclaration> {
        match self {
            Self::Static(Some(declaration)) => Some(declaration),
            Self::Static(None) | Self::Reactive(_) => None,
        }
    }

    fn touch_if_reactive(&self) {
        if let Self::Reactive(signal) = self {
            let _ = signal.get();
        }
    }

    const fn is_reactive(&self) -> bool {
        matches!(self, Self::Reactive(_))
    }
}

/// A storable style entry whose optional declaration may be static or reactive.
///
/// With the `typed-css` feature, checked entries retain the complete
/// `leptos_css::CheckedDeclaration`. Raw property/value strings can enter only through
/// the explicit `_unchecked` constructors or the parsing escape hatch.
#[derive(Clone, Debug)]
pub struct StyleEntry {
    source: StyleDeclarationSource,
}

impl StyleEntry {
    pub(crate) fn static_optional(declaration: Option<StyleDeclaration>) -> Self {
        Self {
            source: StyleDeclarationSource::Static(declaration),
        }
    }

    pub(crate) fn reactive_signal(value: Signal<Option<StyleDeclaration>>) -> Self {
        Self {
            source: StyleDeclarationSource::Reactive(value),
        }
    }

    pub(crate) fn from_declaration(declaration: StyleDeclaration) -> Self {
        Self::static_optional(Some(declaration))
    }

    /// Create an entry from a reactive optional unchecked value.
    ///
    /// When the signal returns `None`, this entry is excluded from the output. Prefer the
    /// checked property-first APIs when the `typed-css` feature is enabled.
    pub fn reactive_unchecked(
        property: impl IntoUncheckedPropertyName,
        value: impl Into<Signal<Option<UncheckedStyleValue>>>,
    ) -> Self {
        let property = normalize_style_property(property.into_unchecked_property_name());
        let value = value.into();
        Self::reactive_signal(Signal::derive(move || {
            value
                .get()
                .map(|value| StyleDeclaration::unchecked(property.clone(), value))
        }))
    }

    /// Create an entry from an always-present unchecked property/value pair.
    ///
    /// Prefer the checked property-first APIs when the `typed-css` feature is enabled.
    pub fn always_unchecked(
        property: impl IntoUncheckedPropertyName,
        value: impl Into<UncheckedStyleValue>,
    ) -> Self {
        Self::from_declaration(StyleDeclaration::unchecked(
            property.into_unchecked_property_name(),
            value.into(),
        ))
    }

    /// Parse a raw CSS declaration such as `"color: red"` into a style entry.
    ///
    /// This parser is an explicit unchecked escape hatch for existing declaration strings. It
    /// rejects missing properties, missing values, and strings without a `:` separator.
    ///
    /// # Errors
    ///
    /// Returns [`ParseStyleEntryError`] if the declaration is not a `property: value` pair.
    ///
    /// # Example
    ///
    /// ```
    /// use leptos_styles::StyleEntry;
    ///
    /// let entry = StyleEntry::parse("color: red").unwrap();
    /// assert_eq!(entry.static_property_name(), Some("color"));
    /// assert!(StyleEntry::parse("color").is_err());
    /// ```
    pub fn parse(declaration: &str) -> Result<Self, ParseStyleEntryError> {
        let (property, value) = parse_style_declaration(declaration)?;
        Ok(Self::always_unchecked(property, value))
    }

    /// Return the property name when this entry contains a present static declaration.
    ///
    /// Reactive entries return `None` because their declaration, including its property, can
    /// change whenever the source is evaluated.
    #[must_use]
    pub fn static_property_name(&self) -> Option<&str> {
        self.source
            .static_declaration()
            .map(StyleDeclaration::property_name)
    }

    pub(crate) fn resolve(&self) -> Option<Cow<'_, StyleDeclaration>> {
        self.source.resolve()
    }

    pub(crate) fn touch_reactive_dependencies(&self) {
        self.source.touch_if_reactive();
    }

    pub(crate) const fn is_reactive(&self) -> bool {
        self.source.is_reactive()
    }
}

#[cfg(feature = "typed-css")]
impl From<leptos_css::CheckedDeclaration> for StyleEntry {
    fn from(declaration: leptos_css::CheckedDeclaration) -> Self {
        Self::from_declaration(StyleDeclaration::Checked(declaration))
    }
}

impl FromStr for StyleEntry {
    type Err = ParseStyleEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for StyleEntry {
    type Error = ParseStyleEntryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for StyleEntry {
    type Error = ParseStyleEntryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::{IntoUncheckedPropertyName, StyleEntry, UncheckedPropertyName};
    use crate::Styles;

    mod parsing {
        use super::*;

        #[test]
        fn parse_recovers_property_value_pair() {
            let entry = StyleEntry::parse("padding: 10px").expect("style declaration should parse");
            let styles: Styles = entry.into();
            assert_that!(styles.to_style_string()).is_equal_to("padding:10px;".to_string());
        }

        #[test]
        fn parse_tolerates_trailing_semicolon() {
            let entry = StyleEntry::parse("margin: 5px;").expect("style declaration should parse");
            let styles: Styles = entry.into();
            assert_that!(styles.to_style_string()).is_equal_to("margin:5px;".to_string());
        }

        #[test]
        fn parse_rejects_missing_colon() {
            let err = StyleEntry::parse("margin 5px").expect_err("style declaration should fail");
            assert_that!(err.reason()).is_equal_to("missing ':' separator");
        }

        #[test]
        fn parse_rejects_empty_property() {
            let err = StyleEntry::parse(": 5px").expect_err("style declaration should fail");
            assert_that!(err.reason()).is_equal_to("missing property name");
        }

        #[test]
        fn parse_rejects_empty_value() {
            let err = StyleEntry::parse("margin: ;").expect_err("style declaration should fail");
            assert_that!(err.reason()).is_equal_to("missing property value");
        }
    }

    mod normalization {
        use super::*;

        #[test]
        fn standard_property_names_are_trimmed_and_lowercased() {
            let styles = Styles::builder()
                .with_unchecked(" Color ", "red")
                .with_unchecked("TOUCH-action", "none")
                .build();

            assert_that!(styles.to_style_string())
                .is_equal_to("color:red;touch-action:none;".to_string());
        }

        #[test]
        fn custom_properties_preserve_case_after_trimming() {
            let styles = Styles::new().add_unchecked(" --ThemeAccent ", "tomato");
            assert_that!(styles.to_style_string()).is_equal_to("--ThemeAccent:tomato;".to_string());
        }

        #[test]
        fn unchecked_property_name_conversion_accepts_custom_marker_types() {
            struct TouchAction;

            impl IntoUncheckedPropertyName for TouchAction {
                fn into_unchecked_property_name(self) -> UncheckedPropertyName {
                    "touch-action".into()
                }
            }

            let styles =
                Styles::new().add_declaration(StyleEntry::always_unchecked(TouchAction, "none"));
            assert_that!(styles.to_style_string()).is_equal_to("touch-action:none;".to_string());
        }
    }
}
