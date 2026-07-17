use std::{borrow::Cow, str::FromStr};

use smallvec::SmallVec;

use crate::{
    IntoOptionalStyleDeclaration, IntoOptionalUncheckedStyleValue, IntoUncheckedPropertyName,
    ParseStyleEntryError, StyleEntry, UncheckedStyleValue,
    convert::reactive_declaration,
    style_entry::StyleDeclaration,
    style_list::{StyleList, StylePriority},
};

/// Leptos component-prop-utility to drill down a list of inline styles.
///
/// # Duplicate Handling
///
/// Duplicate style properties are allowed and will not be deduplicated. In debug builds, a
/// warning is logged when the same statically-known property is added multiple times at normal
/// priority. Reactive complete declarations may change property, so their conflicts are resolved
/// only from the declaration snapshot used for rendering.
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
///         <NeedingStyles styles=styles.add_unchecked("margin", "10px")/>
///     }
/// }
///
/// /// Root component defines the initial styles using a builder pattern.
/// #[component]
/// fn ProvidingStyles() -> impl IntoView {
///     let (color, _) = signal(Some("blue".to_string()));
///     view! {
///         <ExtendingStyles styles=Styles::new().add_unchecked("color", "red")/>
///         <ExtendingStyles styles=Styles::builder()
///             .with_unchecked("padding", "10px")
///             .with_optional_unchecked("color", move || color.get())
///             .build()
///         />
///     }
/// }
/// ```
///
/// With the `typed-css` cargo feature enabled, the same component pattern accepts complete
/// checked declarations:
///
/// ```rust
/// # #[cfg(feature = "typed-css")]
/// # fn _typed_demo() {
/// use leptos::prelude::*;
/// use leptos_styles::{Styles, css::{Padding, px}, property::PaddingProperty};
///
/// let styles = Styles::builder()
///     .with(PaddingProperty.declare(Padding::all(px(10))))
///     .build();
/// assert_eq!(styles.to_style_string(), "padding:10px;");
/// # }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Styles {
    pub(crate) styles: StyleList,
}

/// Builder for [`Styles`]. Obtain one via [`Styles::builder`] and finish with [`Self::build`].
#[derive(Clone, Debug, Default)]
pub struct StylesBuilder {
    styles: StyleList,
}

impl StylesBuilder {
    /// Add one complete pre-built declaration to the builder.
    #[must_use]
    pub fn with(mut self, declaration: impl Into<StyleEntry>) -> Self {
        self.styles.push(declaration.into());
        self
    }

    /// Add a conditionally-present complete declaration to the builder.
    #[must_use]
    pub fn with_optional<D>(mut self, declaration: impl IntoOptionalStyleDeclaration<D>) -> Self {
        self.styles
            .push(declaration.into_optional_declaration_entry());
        self
    }

    /// Add an always-present reactive complete declaration to the builder.
    ///
    /// The closure form is portable across stable and nightly Rust. Use [`Self::with_optional`]
    /// when the declaration can be absent.
    #[must_use]
    pub fn with_reactive<D>(mut self, declaration: impl Fn() -> D + Send + Sync + 'static) -> Self
    where
        D: Into<StyleEntry>,
    {
        self.styles.push(reactive_declaration(declaration));
        self
    }

    /// Add one always-present unchecked property/value pair.
    #[must_use]
    pub fn with_unchecked(
        mut self,
        property: impl IntoUncheckedPropertyName,
        value: impl Into<UncheckedStyleValue>,
    ) -> Self {
        self.styles
            .push(StyleEntry::always_unchecked(property, value));
        self
    }

    /// Add one conditionally-present unchecked property/value pair.
    #[must_use]
    pub fn with_optional_unchecked<V>(
        mut self,
        property: impl IntoUncheckedPropertyName,
        value: impl IntoOptionalUncheckedStyleValue<V>,
    ) -> Self {
        self.styles
            .push(value.into_optional_unchecked_entry(property.into_unchecked_property_name()));
        self
    }

    /// Add multiple complete pre-built declarations to the builder.
    #[must_use]
    pub fn with_declarations<D: Into<StyleEntry>>(
        mut self,
        iter: impl IntoIterator<Item = D>,
    ) -> Self {
        for declaration in iter {
            self.styles.push(declaration.into());
        }
        self
    }

    /// Build the style container.
    #[must_use]
    pub fn build(self) -> Styles {
        Styles {
            styles: self.styles,
        }
    }
}

impl Styles {
    /// Create an empty style container.
    #[must_use]
    pub fn new() -> Self {
        Self {
            styles: StyleList::default(),
        }
    }

    /// Create a builder for [`Styles`].
    #[must_use]
    pub fn builder() -> StylesBuilder {
        StylesBuilder::default()
    }

    /// Add one complete pre-built declaration to this style-list.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, declaration: impl Into<StyleEntry>) -> Self {
        self.styles.push(declaration.into());
        self
    }

    /// Add one conditionally-present complete declaration to this style-list.
    #[must_use]
    pub fn add_optional<D>(mut self, declaration: impl IntoOptionalStyleDeclaration<D>) -> Self {
        self.styles
            .push(declaration.into_optional_declaration_entry());
        self
    }

    /// Add an always-present reactive complete declaration to this style-list.
    ///
    /// The closure form is portable across stable and nightly Rust. Use [`Self::add_optional`]
    /// when the declaration can be absent.
    #[must_use]
    pub fn add_reactive<D>(mut self, declaration: impl Fn() -> D + Send + Sync + 'static) -> Self
    where
        D: Into<StyleEntry>,
    {
        self.styles.push(reactive_declaration(declaration));
        self
    }

    /// Add one always-present unchecked property/value pair.
    #[must_use]
    pub fn add_unchecked(
        mut self,
        property: impl IntoUncheckedPropertyName,
        value: impl Into<UncheckedStyleValue>,
    ) -> Self {
        self.styles
            .push(StyleEntry::always_unchecked(property, value));
        self
    }

    /// Add one conditionally-present unchecked property/value pair.
    #[must_use]
    pub fn add_optional_unchecked<V>(
        mut self,
        property: impl IntoUncheckedPropertyName,
        value: impl IntoOptionalUncheckedStyleValue<V>,
    ) -> Self {
        self.styles
            .push(value.into_optional_unchecked_entry(property.into_unchecked_property_name()));
        self
    }

    /// Add multiple complete declarations to this style-list.
    #[must_use]
    pub fn add_declarations<D: Into<StyleEntry>>(
        mut self,
        iter: impl IntoIterator<Item = D>,
    ) -> Self {
        for declaration in iter {
            self.styles.push(declaration.into());
        }
        self
    }

    /// Parses a raw CSS string containing one or more `property: value;` declarations.
    ///
    /// Empty declarations between semicolons are skipped. Each non-empty declaration must
    /// be a valid `property: value` pair; otherwise a [`ParseStyleEntryError`] is returned
    /// and no partial state is retained.
    ///
    /// This is intended as an escape hatch for copy-pasted CSS. Prefer checked property selectors
    /// with [`Styles::builder`] when authoring supported styles, or the explicit `_unchecked`
    /// methods when deliberately using unsupported CSS.
    ///
    /// # Errors
    ///
    /// Returns the first [`ParseStyleEntryError`] encountered while parsing declarations.
    ///
    /// # Example
    ///
    /// ```
    /// use leptos_styles::Styles;
    ///
    /// let styles = Styles::parse_css("color: red; padding: 8px;").unwrap();
    /// assert_eq!(styles.to_style_string(), "color:red;padding:8px;");
    /// ```
    pub fn parse_css(css: &str) -> Result<Self, ParseStyleEntryError> {
        let mut styles = Self::new();
        for piece in css.split(';') {
            if piece.trim().is_empty() {
                continue;
            }
            styles.styles.push(StyleEntry::parse(piece)?);
        }
        Ok(styles)
    }

    /// Merge `other` (lower-priority) styles into `self` (higher-priority).
    ///
    /// Entries from `other` are assigned a lower-priority fallback group while retaining their
    /// order and any existing fallback relationships. Property conflicts are decided after every
    /// reactive entry has resolved to one complete declaration snapshot. This allows either layer
    /// to change property safely: a lower-priority declaration renders whenever no present
    /// higher-priority declaration currently resolves to the same property.
    ///
    /// # Example
    ///
    /// ```
    /// use leptos_styles::Styles;
    ///
    /// let user = Styles::builder().with_unchecked("color", "red").build();
    /// let theme = Styles::builder()
    ///     .with_unchecked("color", "blue")
    ///     .with_unchecked("padding", "8px")
    ///     .build();
    ///
    /// // `user` wins on color; `theme.padding` flows through unchanged.
    /// let merged = user.merge(theme).to_style_string();
    /// assert!(merged.contains("color:red"));
    /// assert!(merged.contains("padding:8px"));
    /// assert!(!merged.contains("color:blue"));
    /// ```
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        let fallback_group = self.styles.next_fallback_group();

        for entry in other.styles.into_entries() {
            let group = match entry.priority {
                StylePriority::Normal => fallback_group,
                StylePriority::Fallback(group) => fallback_group.saturating_add(group),
            };
            self.styles.push_fallback(entry.style, group);
        }
        self
    }

    /// Appends all defined styles to the given string buffer.
    ///
    /// Entries with `None` values are excluded from the output.
    ///
    /// For each property, the lowest-numbered priority group that currently resolves to
    /// `Some(_)` wins; entries at that group render in their original push order, while
    /// entries at higher (less-priority) groups for the same property are skipped. This
    /// covers chained-merge fallback chains as well as the case where a later `Normal`
    /// entry is added on top of a `Fallback` chain for the same property.
    ///
    /// The internal declaration snapshot and winning-property list remain inline for up to four
    /// present entries/properties. Larger containers spill those small vectors to the heap.
    pub fn write_style_string(&self, buf: &mut String) {
        let resolved: SmallVec<[(Cow<'_, StyleDeclaration>, StylePriority); 4]> = self
            .styles
            .iter_entries()
            .filter_map(|(entry, priority)| {
                entry.resolve().map(|declaration| (declaration, priority))
            })
            .collect();
        let mut winning = SmallVec::<[(&str, u16); 4]>::new();

        for (declaration, priority) in &resolved {
            let property = declaration.property_name();
            let group = priority.group();
            match winning
                .iter_mut()
                .find(|(candidate, _)| *candidate == property)
            {
                Some((_, current)) if group < *current => *current = group,
                Some(_) => {}
                None => winning.push((property, group)),
            }
        }

        for (declaration, priority) in &resolved {
            let property = declaration.property_name();
            let group = priority.group();
            if winning
                .iter()
                .any(|(candidate, winning_group)| *candidate == property && *winning_group == group)
            {
                declaration.write_to(buf);
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

    /// Returns `true` if any style entry in this container resolves through a reactive signal.
    ///
    /// When this returns `false`, the container can be rendered as a static `style` attribute
    /// without subscribing to the reactive graph.
    #[must_use]
    pub fn is_reactive(&self) -> bool {
        self.styles.is_reactive()
    }
}

impl FromStr for Styles {
    type Err = ParseStyleEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_css(s)
    }
}

impl TryFrom<&str> for Styles {
    type Error = ParseStyleEntryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_css(value)
    }
}

impl TryFrom<String> for Styles {
    type Error = ParseStyleEntryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse_css(&value)
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use crate::Styles;

    mod construction {
        use super::*;

        #[test]
        fn new_renders_nothing() {
            let styles = Styles::new();
            assert_that!(styles.to_style_string()).is_equal_to(String::new());
        }

        #[test]
        fn add_chains_in_insertion_order() {
            let styles = Styles::new()
                .add_unchecked("color", "green")
                .add_unchecked("font-size", "14px");
            assert_that!(styles.to_style_string())
                .is_equal_to("color:green;font-size:14px;".to_string());
        }

        #[test]
        fn builder_chains_in_insertion_order() {
            let styles = Styles::builder()
                .with_unchecked("margin", "10px")
                .with_unchecked("display", "flex")
                .build();
            assert_that!(styles.to_style_string())
                .is_equal_to("margin:10px;display:flex;".to_string());
        }

        #[test]
        fn add_declarations_extends_with_iterator() {
            let styles = Styles::new()
                .add_unchecked("display", "grid")
                .add_declarations([
                    crate::StyleEntry::always_unchecked("color", "blue"),
                    crate::StyleEntry::always_unchecked("padding", "20px"),
                ]);

            assert_that!(styles.to_style_string())
                .is_equal_to("display:grid;color:blue;padding:20px;".to_string());
        }

        #[test]
        fn mixed_static_and_optional_skips_none_entries() {
            let styles = Styles::builder()
                .with_unchecked("color", "red")
                .with_optional_unchecked("padding", None::<&str>)
                .with_unchecked("margin", "10px")
                .build();
            assert_that!(styles.to_style_string())
                .is_equal_to("color:red;margin:10px;".to_string());
        }

        #[test]
        fn chained_adds_drill_through_layers() {
            let initial = Styles::new().add_unchecked("color", "red");
            let extended = initial.add_unchecked("margin", "5px");
            let drilled = extended.add_unchecked("padding", "10px");

            assert_that!(drilled.to_style_string())
                .is_equal_to("color:red;margin:5px;padding:10px;".to_string());
        }
    }

    mod parsing {
        use super::*;

        #[test]
        fn parse_css_single_declaration() {
            let styles = Styles::parse_css("color: red").expect("css should parse");
            assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());
        }

        #[test]
        fn parse_css_multiple_declarations() {
            let styles = Styles::parse_css("color: red; padding: 1rem; margin: 0")
                .expect("css should parse");
            assert_that!(styles.to_style_string())
                .is_equal_to("color:red;padding:1rem;margin:0;".to_string());
        }

        #[test]
        fn parse_css_skips_empty_declarations() {
            let styles =
                Styles::parse_css(";;color: red;;padding: 1rem;;").expect("css should parse");
            assert_that!(styles.to_style_string())
                .is_equal_to("color:red;padding:1rem;".to_string());
        }

        #[test]
        fn parse_css_empty_input_yields_empty_styles() {
            let styles = Styles::parse_css("   ; ;  ").expect("css should parse");
            assert_that!(styles.to_style_string()).is_equal_to(String::new());
        }

        #[test]
        fn parse_css_propagates_declaration_error() {
            let err = Styles::parse_css("color: red; invalid").expect_err("css should fail");
            assert_that!(err.reason()).is_equal_to("missing ':' separator");
        }

        #[test]
        fn parse_css_via_try_from_str() {
            let styles: Styles = "color: blue; padding: 4px"
                .try_into()
                .expect("css should parse");
            assert_that!(styles.to_style_string())
                .is_equal_to("color:blue;padding:4px;".to_string());
        }

        #[test]
        fn parse_css_via_from_str() {
            let styles: Styles = "color: green".parse().expect("css should parse");
            assert_that!(styles.to_style_string()).is_equal_to("color:green;".to_string());
        }
    }

    mod merge {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        use super::*;

        #[test]
        fn merge_treats_property_names_case_insensitively() {
            let hook_styles = Styles::new().add_unchecked("touch-action", "none");
            let user_styles = Styles::new().add_unchecked("Touch-Action", "auto");

            assert_that!(hook_styles.merge(user_styles).to_style_string())
                .is_equal_to("touch-action:none;".to_string());
        }

        #[test]
        fn merge_allows_static_none_fallback() {
            let hook_styles = Styles::new().add_optional_unchecked("color", None::<&str>);
            let user_styles = Styles::new().add_unchecked("Color", "gray");

            assert_that!(hook_styles.merge(user_styles).to_style_string())
                .is_equal_to("color:gray;".to_string());
        }

        #[test]
        fn merge_reactive_optional_fallback_toggles() {
            let use_hook_color = Arc::new(AtomicBool::new(true));
            let use_hook_color_for_style = Arc::clone(&use_hook_color);
            let hook_styles = Styles::new().add_optional_unchecked("color", move || {
                if use_hook_color_for_style.load(Ordering::Relaxed) {
                    Some("orange")
                } else {
                    None
                }
            });
            let user_styles = Styles::new().add_unchecked("Color", "gray");
            let styles = hook_styles.merge(user_styles);

            assert_that!(styles.to_style_string()).is_equal_to("color:orange;".to_string());

            use_hook_color.store(false, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("color:gray;".to_string());

            use_hook_color.store(true, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("color:orange;".to_string());
        }

        #[test]
        fn merge_preserves_normal_duplicate_last_wins_with_fallback() {
            let use_hook_color = Arc::new(AtomicBool::new(true));
            let use_hook_color_for_style = Arc::clone(&use_hook_color);
            let hook_styles = Styles::new()
                .add_optional_unchecked("color", move || {
                    if use_hook_color_for_style.load(Ordering::Relaxed) {
                        Some("orange")
                    } else {
                        None
                    }
                })
                .add_unchecked("color", "red");
            let user_styles = Styles::new().add_unchecked("Color", "gray");
            let styles = hook_styles.merge(user_styles);

            assert_that!(styles.to_style_string())
                .is_equal_to("color:orange;color:red;".to_string());

            use_hook_color.store(false, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());
        }

        #[test]
        fn merge_same_batch_fallback_duplicates_preserve_order() {
            let hook_styles = Styles::new().add_optional_unchecked("color", None::<&str>);
            let user_styles = Styles::new()
                .add_unchecked("Color", "gray")
                .add_unchecked("Color", "blue");

            assert_that!(hook_styles.merge(user_styles).to_style_string())
                .is_equal_to("color:gray;color:blue;".to_string());
        }

        #[test]
        fn merge_lower_batch_fallback_does_not_override_higher_batch_fallback() {
            let hook_styles = Styles::new().add_optional_unchecked("color", None::<&str>);
            let user_styles = Styles::new().add_unchecked("Color", "gray");
            let theme_styles = Styles::new().add_unchecked("Color", "blue");

            assert_that!(
                hook_styles
                    .merge(user_styles)
                    .merge(theme_styles)
                    .to_style_string()
            )
            .is_equal_to("color:gray;".to_string());
        }

        #[test]
        fn merge_static_some_suppresses_fallback() {
            let hook_styles = Styles::new().add_unchecked("color", "orange");
            let user_styles = Styles::new().add_unchecked("Color", "gray");

            assert_that!(hook_styles.merge(user_styles).to_style_string())
                .is_equal_to("color:orange;".to_string());
        }

        #[test]
        fn normal_added_after_merge_fallback_overrides_fallback() {
            // Pushing a `Normal` for the same property after a fallback chain has been merged
            // in must shadow that fallback chain rather than render alongside it. The duplicate
            // push triggers the debug-build warning in `StyleList::push`, which is the intended
            // surface for this kind of conflict.
            let hook_styles = Styles::new().add_optional_unchecked("color", None::<&str>);
            let merged = hook_styles
                .merge(Styles::new().add_unchecked("color", "fallback"))
                .add_unchecked("color", "user");

            assert_that!(merged.to_style_string()).is_equal_to("color:user;".to_string());
        }

        #[test]
        fn merge_empty_on_empty_yields_empty() {
            let merged = Styles::new().merge(Styles::new());
            assert_that!(merged.to_style_string()).is_equal_to(String::new());
            assert_that!(merged.is_reactive()).is_false();
        }

        #[test]
        fn merge_empty_into_non_empty_preserves_self() {
            let base = Styles::builder().with_unchecked("color", "red").build();
            let merged = base.merge(Styles::new());
            assert_that!(merged.to_style_string()).is_equal_to("color:red;".to_string());
        }

        #[test]
        fn merge_non_empty_into_empty_appends_other() {
            let merged =
                Styles::new().merge(Styles::builder().with_unchecked("color", "red").build());
            assert_that!(merged.to_style_string()).is_equal_to("color:red;".to_string());
        }
    }

    mod reactivity {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        use super::*;

        #[test]
        fn is_reactive_is_false_for_purely_static_container() {
            let styles = Styles::builder()
                .with_unchecked("color", "red")
                .with_optional_unchecked("padding", Some("8px"))
                .with_optional_unchecked("margin", None::<&str>)
                .build();
            assert_that!(styles.is_reactive()).is_false();
        }

        #[test]
        fn is_reactive_is_true_when_an_optional_closure_entry_is_present() {
            let toggle = Arc::new(AtomicBool::new(true));
            let toggle_for_style = Arc::clone(&toggle);
            let styles = Styles::builder()
                .with_unchecked("color", "red")
                .with_optional_unchecked("width", move || {
                    Some(if toggle_for_style.load(Ordering::Relaxed) {
                        "50%"
                    } else {
                        "100%"
                    })
                })
                .build();
            assert_that!(styles.is_reactive()).is_true();
        }

        #[test]
        fn is_reactive_is_true_when_a_reactive_optional_is_present() {
            let toggle = Arc::new(AtomicBool::new(true));
            let toggle_for_style = Arc::clone(&toggle);
            let styles = Styles::builder()
                .with_optional_unchecked("color", move || {
                    toggle_for_style.load(Ordering::Relaxed).then_some("red")
                })
                .build();
            assert_that!(styles.is_reactive()).is_true();
        }
    }

    #[cfg(feature = "typed-css")]
    mod typed_css_rendering {
        use std::sync::{
            Arc,
            atomic::{AtomicBool, AtomicUsize, Ordering},
        };

        use super::*;
        use crate::{
            css::{
                CssColor, CssColorName, CssCustomProperty, LengthPercentageAuto,
                LengthPercentageCalculation, Margin, NonNegativeLengthPercentage, Padding, Size,
                css_custom_property, em, pct, px, rem, var, vh, vw,
            },
            property::{
                BackgroundColorProperty, ColorProperty, HeightProperty, MarginProperty,
                PaddingProperty, RightProperty, WidthProperty,
            },
        };

        css_custom_property!(ACCENT_COLOR: CssColor = "--AccentColor");

        #[test]
        fn checked_custom_properties_and_vars_render_through_the_pipeline() {
            let missing_color = CssCustomProperty::<CssColor>::new("--MissingColor");

            let styles = Styles::builder()
                .with(ACCENT_COLOR.declare(CssColor::Named(CssColorName::Fuchsia)))
                .with(
                    ColorProperty.declare(var(&ACCENT_COLOR, CssColor::Named(CssColorName::Black))),
                )
                .with(BackgroundColorProperty.declare(var(
                    &missing_color,
                    CssColor::Named(CssColorName::CurrentColor),
                )))
                .build();

            assert_that!(styles.to_style_string()).is_equal_to(
                "--AccentColor:fuchsia;color:var(--AccentColor, black);background-color:var(--MissingColor, currentcolor);"
                    .to_string(),
            );
        }

        #[test]
        fn checked_calculations_render_through_the_styles_pipeline() {
            let styles =
                Styles::builder()
                    .with(WidthProperty.declare(Size::Calculation(
                        LengthPercentageCalculation::new(pct(100) - px(20)),
                    )))
                    .with(HeightProperty.declare(Size::Calculation(
                        LengthPercentageCalculation::new(vh(50) + rem(1)),
                    )))
                    .build();

            assert_that!(styles.to_style_string())
                .is_equal_to("width:calc(100% - 20px);height:calc(50vh + 1rem);".to_string());
        }

        #[test]
        fn typed_units_render_through_the_styles_pipeline() {
            let styles = Styles::builder()
                .with(WidthProperty.declare(Size::from(NonNegativeLengthPercentage::new(pct(50)))))
                .with(HeightProperty.declare(Size::from(NonNegativeLengthPercentage::new(vh(40)))))
                .with(PaddingProperty.declare(Padding::all(em(1.5))))
                .with(MarginProperty.declare(Margin::All(LengthPercentageAuto::from(rem(2)))))
                .with(RightProperty.declare(LengthPercentageAuto::from(vw(10))))
                .build();

            assert_that!(styles.to_style_string()).is_equal_to(
                "width:50%;height:40vh;padding:1.5em;margin:2rem;right:10vw;".to_string(),
            );
        }

        #[test]
        fn reactive_typed_value_renders_through_signal() {
            let styles = Styles::builder()
                .with_unchecked("color", "red")
                .with_optional(move || {
                    Some(
                        WidthProperty
                            .declare(Size::from(NonNegativeLengthPercentage::new(pct(50)))),
                    )
                })
                .build();
            assert_that!(styles.is_reactive()).is_true();
            assert_that!(styles.to_style_string()).is_equal_to("color:red;width:50%;".to_string());
        }

        #[test]
        fn reactive_complete_declaration_can_change_property_and_is_resolved_once() {
            let use_background = Arc::new(AtomicBool::new(false));
            let resolutions = Arc::new(AtomicUsize::new(0));
            let use_background_for_declaration = Arc::clone(&use_background);
            let resolutions_for_declaration = Arc::clone(&resolutions);

            let styles = Styles::builder()
                .with_reactive(move || {
                    resolutions_for_declaration.fetch_add(1, Ordering::Relaxed);
                    let color = CssColor::Named(CssColorName::Red);
                    if use_background_for_declaration.load(Ordering::Relaxed) {
                        BackgroundColorProperty.declare(color)
                    } else {
                        ColorProperty.declare(color)
                    }
                })
                .build();

            assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());
            assert_that!(resolutions.load(Ordering::Relaxed)).is_equal_to(1);

            use_background.store(true, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("background-color:red;".to_string());
            assert_that!(resolutions.load(Ordering::Relaxed)).is_equal_to(2);
        }
    }
}
