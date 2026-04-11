use leptos::typed_builder::TypedBuilder;
use smallvec::SmallVec;

use crate::{
    IntoOptionalStyleEntryValue, IntoStyleEntryValue, IntoStyleProperty, StyleEntry, StyleList,
    StyleProperty,
};

/// Leptos component-prop-utility to drill down a list of inline styles.
///
/// # Duplicate Handling
///
/// Duplicate style properties are allowed and will not be deduplicated. However, in debug builds,
/// a warning will be logged when the same property name is added multiple times. This behavior
/// helps identify potential bugs where styles are unintentionally added twice.
///
/// The final style string produced by [`Styles::to_style_string`] may contain repeated
/// properties if duplicates were added.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptos_styles::Styles;
///
/// /// The lowest-level component renders the style-list into an actual HTML element.
/// #[component]
/// fn NeedingStyles(
///     #[prop(into, optional)] styles: Styles,
/// ) -> impl IntoView {
///     view! {
///         <div style=styles/>
///     }
/// }
///
/// /// Components sitting in the middle can add their own styles.
/// #[component]
/// fn ExtendingStyles(
///     #[prop(into, optional)] styles: Styles,
/// ) -> impl IntoView {
///     view! {
///         <NeedingStyles styles=styles.add("margin", "10px")/>
///     }
/// }
///
/// /// Root component defines the initial styles using a builder pattern.
/// #[component]
/// fn ProvidingStyles() -> impl IntoView {
///     let (color, _) = signal(Some("blue".to_string()));
///     view! {
///         <ExtendingStyles styles=("color", "red")/>
///         <ExtendingStyles styles=Styles::builder()
///             .with("padding", "10px")
///             .with_optional("color", color)
///             .build()
///         />
///     }
/// }
/// ```
#[derive(Clone, Debug, Default, TypedBuilder)]
#[builder(crate_module_path = ::leptos::typed_builder)]
#[builder(mutators(
    pub fn with(&mut self, property: impl IntoStyleProperty, value: impl IntoStyleEntryValue) {
        self.styles.push(value.into_style_entry(property.into_property()));
    }

    pub fn with_optional(&mut self, property: impl IntoStyleProperty, value: impl IntoOptionalStyleEntryValue) {
        self.styles.push(value.into_style_entry(property.into_property()));
    }

    pub fn with_entry(&mut self, style: impl Into<StyleEntry>) {
        self.styles.push(style.into());
    }

    pub fn with_all<S: Into<StyleEntry>>(&mut self, iter: impl Iterator<Item = S>) {
        for style in iter {
            self.styles.push(style.into());
        }
    }
))]
pub struct Styles {
    #[builder(via_mutators)]
    pub(crate) styles: StyleList,
    #[builder(default)]
    fallback_properties: Vec<StyleProperty>,
}

impl Styles {
    #[must_use]
    pub fn new() -> Self {
        Self {
            styles: StyleList::default(),
            fallback_properties: Vec::new(),
        }
    }

    /// Add one additional style to this style-list.
    #[must_use]
    pub fn add(
        mut self,
        property: impl IntoStyleProperty,
        value: impl IntoStyleEntryValue,
    ) -> Self {
        self.styles
            .push(value.into_style_entry(property.into_property()));
        self
    }

    /// Add one additional style with a conditionally-present value.
    ///
    /// When the value resolves to `None`, the style is excluded from the output.
    #[must_use]
    pub fn add_optional(
        mut self,
        property: impl IntoStyleProperty,
        value: impl IntoOptionalStyleEntryValue,
    ) -> Self {
        self.styles
            .push(value.into_style_entry(property.into_property()));
        self
    }

    /// Add one additional pre-built style entry to this style-list.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add_entry(mut self, entry: impl Into<StyleEntry>) -> Self {
        self.styles.push(entry.into());
        self
    }

    /// Add multiple styles to this style-list.
    pub fn add_all<S: Into<StyleEntry>>(&mut self, iter: impl Iterator<Item = S>) {
        for style in iter {
            self.styles.push(style.into());
        }
    }

    /// Merge `other` (lower-priority) styles into `self` (higher-priority).
    ///
    /// `self` entries take precedence when they currently resolve to a value. If
    /// a higher-priority entry for the same property resolves to `None`, the
    /// lower-priority `other` entry can render as a fallback.
    ///
    /// Non-conflicting entries from `other` are appended to `self`.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        for entry in other.styles.into_entries() {
            if let Some(conflicting) = self.styles.find_by_property(entry.property()) {
                if conflicting.is_same_static_value(&entry) {
                    continue;
                }

                if conflicting.is_static_some() {
                    #[cfg(debug_assertions)]
                    {
                        tracing::warn!(
                            "Style property '{}' provided by the user conflicts with a hook-managed style and will be ignored. {}",
                            entry.property(),
                            std::backtrace::Backtrace::force_capture()
                        );
                    }
                } else {
                    self.mark_fallback_property(entry.property());
                    self.styles.push_fallback(entry);
                }
            } else {
                self.styles.push(entry);
            }
        }
        self
    }

    /// Appends all defined styles to the given string buffer.
    ///
    /// Entries with `None` values are excluded from the output.
    /// This method is zero-allocation when the buffer has sufficient capacity.
    pub fn write_style_string(&self, buf: &mut String) {
        if self.fallback_properties.is_empty() {
            for entry in self.styles.iter() {
                entry.write_declaration_to(buf);
            }
        } else {
            let mut emitted_fallback_properties = SmallVec::<[&str; 4]>::new();

            for entry in self.styles.iter() {
                let property = entry.property();
                if self.is_fallback_property(property) {
                    if emitted_fallback_properties.iter().any(|it| *it == property) {
                        continue;
                    }

                    if entry.write_declaration_to(buf) {
                        emitted_fallback_properties.push(property);
                    }
                } else {
                    entry.write_declaration_to(buf);
                }
            }
        }
    }

    /// Reactively combines all defined styles into one semicolon-separated String.
    ///
    /// Entries with `None` values are excluded from the output.
    ///
    /// Note: Prefer using `style=styles` directly (via `IntoStyle`) for better performance,
    /// as it reuses the string buffer across reactive updates.
    #[must_use]
    pub fn to_style_string(&self) -> String {
        let mut s = String::new();
        self.write_style_string(&mut s);
        s
    }

    pub(crate) fn touch_reactive_dependencies(&self) {
        self.styles.touch_reactive_dependencies();
    }

    fn is_fallback_property(&self, property: &str) -> bool {
        self.fallback_properties
            .iter()
            .any(|candidate| candidate == property)
    }

    fn mark_fallback_property(&mut self, property: &str) {
        if !self.is_fallback_property(property) {
            self.fallback_properties
                .push(std::borrow::Cow::Owned(property.to_string()));
        }
    }
}
