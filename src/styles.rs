use std::{borrow::Cow, str::FromStr};

use smallvec::SmallVec;

use crate::{
    IntoStyleEntry, InvalidUncheckedDeclaration, StyleEntry, UncheckedDeclaration,
    style_entry::StyleDeclaration,
};

/// The inline style of an element, collected from declarations that components pass on and add to.
///
/// A component takes a `Styles` prop, adds its own declarations with [`Styles::with`], and passes
/// the result on or renders it with `style=styles`. Each declaration can be static, optional, or
/// reactive. [`Styles::with_fallback`] adds a whole layer of lower priority, such as the default
/// styles of a component.
///
/// The same property may appear more than once. Within one layer, declarations render in the
/// order they were added, so a later one can enhance an earlier one, as in
/// `height:100vh;height:100dvh;`.
///
/// A clone copies the static declarations and shares the closures.
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_styles::{
///     Styles,
///     css::{NamedColor, property::{COLOR, DISPLAY, PADDING}, px},
/// };
///
/// #[component]
/// fn Card(#[prop(into, optional)] styles: Styles) -> impl IntoView {
///     view! { <div style=styles.with(PADDING.declare(px(16)))>"Card"</div> }
/// }
///
/// #[component]
/// fn Page() -> impl IntoView {
///     let (highlighted, _set_highlighted) = signal(false);
///     view! {
///         <Card styles=Styles::new()
///             .with(DISPLAY.grid())
///             .with(move || highlighted.get().then(|| COLOR.declare(NamedColor::Crimson)))
///         />
///     }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Styles {
    styles: StyleList,
}

/// The priority of a declaration, as the path of layers that leads to it. See
/// [`Styles::resolve_ranked`].
pub(crate) type Rank = SmallVec<[u16; 2]>;

/// Return the rank of a declaration at `inner` inside an entry of `layer`.
///
/// Trailing zeros are left out, so that a declaration of the first layer of a nested `Styles` has
/// the same rank as the declarations next to it.
fn rank(layer: u16, inner: &[u16]) -> Rank {
    let mut rank = Rank::new();
    rank.push(layer);
    rank.extend_from_slice(inner);
    while rank.last() == Some(&0) {
        rank.pop();
    }
    rank
}

/// The entries of a `Styles` with the priority layer of each.
///
/// Layer 0 holds the entries added with `with`. Each fallback adds layers with higher numbers,
/// which have lower priority. Most elements have at most four entries, so they are stored inline.
#[derive(Clone, Debug, Default)]
struct StyleList {
    entries: SmallVec<[(StyleEntry, u16); 4]>,
    deepest_layer: u16,
    has_reactive: bool,
}

impl StyleList {
    /// Add an entry. An entry that never has a declaration is dropped.
    fn push(&mut self, entry: StyleEntry, layer: u16) {
        if entry.is_empty() {
            return;
        }
        self.has_reactive |= entry.is_reactive();
        self.deepest_layer = self.deepest_layer.max(layer);
        self.entries.push((entry, layer));
    }

    /// Return the first layer below every layer in the list. It is below layer 0 even for an
    /// empty list, so that entries added later with `with` keep the higher priority.
    const fn next_fallback_layer(&self) -> u16 {
        self.deepest_layer.saturating_add(1)
    }
}

impl Styles {
    /// Create an empty style.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a declaration, a `Styles`, an `Option` of either, or a closure that returns one of
    /// them.
    ///
    /// A closure runs again whenever a signal that it reads changes, so the declaration updates
    /// reactively. The first layer of an added `Styles` joins the declarations of `self`, and its
    /// fallback layers stay above the fallback layers of `self`. [`IntoStyleEntry`] describes all
    /// accepted values.
    ///
    /// ```rust
    /// use leptos::prelude::*;
    /// use leptos_styles::{
    ///     Styles,
    ///     css::{NamedColor, pct, property::{COLOR, DISPLAY, GAP, WIDTH}, px},
    /// };
    ///
    /// let (color, set_color) = signal(NamedColor::Red);
    /// let (wide, _set_wide) = signal(false);
    /// let styles = Styles::new()
    ///     .with(DISPLAY.grid())
    ///     .with(Some(GAP.declare(px(8))))
    ///     .with(move || COLOR.declare(color.get()))
    ///     .with(move || wide.get().then(|| WIDTH.declare(pct(100))));
    ///
    /// assert_eq!(styles.to_style_string(), "display:grid;gap:8px;color:red;");
    /// set_color.set(NamedColor::Blue);
    /// assert_eq!(styles.to_style_string(), "display:grid;gap:8px;color:blue;");
    ///
    /// let card = Styles::new().with(styles).with(WIDTH.declare(pct(50)));
    /// assert_eq!(card.to_style_string(), "display:grid;gap:8px;color:blue;width:50%;");
    /// ```
    #[must_use]
    pub fn with<M>(mut self, source: impl IntoStyleEntry<M>) -> Self {
        self.styles.push(source.into_style_entry(), 0);
        self
    }

    /// Add several declarations, `Styles`, `Option`s, or closures of the same kind.
    ///
    /// To combine different kinds in one list, convert each into a [`StyleEntry`] first.
    #[must_use]
    pub fn with_all<M, S: IntoStyleEntry<M>>(
        mut self,
        sources: impl IntoIterator<Item = S>,
    ) -> Self {
        for source in sources {
            self.styles.push(source.into_style_entry(), 0);
        }
        self
    }

    /// Add `fallback` as a layer of lower priority than every declaration in `self`.
    ///
    /// A declaration of the fallback layer renders only while no declaration of a higher layer has
    /// the same property. Because a reactive declaration can change or disappear, the browser
    /// output follows the current values. The order and the layers inside `fallback` are kept.
    ///
    /// Layers of lower priority render first. A shorthand in the fallback, such as `padding`,
    /// therefore never overrides a longhand in `self`, such as `padding-top`.
    ///
    /// ```
    /// use leptos_styles::{Styles, css::{NamedColor, property::{COLOR, PADDING}, px}};
    ///
    /// let user = Styles::new().with(COLOR.declare(NamedColor::Red));
    /// let theme = Styles::new()
    ///     .with(COLOR.declare(NamedColor::Blue))
    ///     .with(PADDING.declare(px(8)));
    ///
    /// // The user's color wins, and the theme's padding fills the gap.
    /// assert_eq!(user.with_fallback(theme).to_style_string(), "padding:8px;color:red;");
    /// ```
    #[must_use]
    pub fn with_fallback(mut self, fallback: impl Into<Self>) -> Self {
        let first_layer = self.styles.next_fallback_layer();
        for (entry, layer) in fallback.into().styles.entries {
            self.styles.push(entry, first_layer.saturating_add(layer));
        }
        self
    }

    /// Parse CSS declarations such as `color: red; padding: 8px`.
    ///
    /// The parser checks each declaration like an [`UncheckedDeclaration`] and skips empty
    /// declarations. A `;` inside a string, `url()`, or a block does not end a declaration. Use
    /// this parser for CSS copied from elsewhere, and prefer checked properties for CSS that
    /// `typed-css` supports.
    ///
    /// # Errors
    ///
    /// Returns an error for the first invalid declaration. The parser then returns no `Styles`.
    ///
    /// ```
    /// use leptos_styles::Styles;
    ///
    /// let styles = Styles::parse("color: red; padding: 8px;").unwrap();
    /// assert_eq!(styles.to_style_string(), "color:red;padding:8px;");
    /// ```
    pub fn parse(css: &str) -> Result<Self, InvalidUncheckedDeclaration> {
        Ok(Self::new().with_all(UncheckedDeclaration::parse_list(css)?))
    }

    /// Read every entry once and visit each declaration with its rank.
    ///
    /// A rank is the path of layers from this `Styles` down to the declaration, which only has
    /// more than one step inside a nested `Styles`. A smaller rank has a higher priority.
    pub(crate) fn resolve_ranked<'a>(
        &'a self,
        visit: &mut dyn FnMut(Cow<'a, StyleDeclaration>, Rank),
    ) {
        for (entry, layer) in &self.styles.entries {
            entry.resolve_each(&mut |declaration, inner| visit(declaration, rank(*layer, inner)));
        }
    }

    /// Visit each declaration with its rank like [`Styles::resolve_ranked`], but move the
    /// declarations out of this `Styles`.
    pub(crate) fn into_ranked(self, visit: &mut dyn FnMut(StyleDeclaration, Rank)) {
        for (entry, layer) in self.styles.entries {
            entry.into_each(&mut |declaration, inner| visit(declaration, rank(layer, inner)));
        }
    }

    /// Read every entry once and pass the declarations that win their property to `f`.
    ///
    /// For each property, the declarations with the highest priority win. The winning
    /// declarations are ordered from the lowest priority to the highest, and in the order they
    /// were added within one priority.
    pub(crate) fn with_winning<R>(&self, f: impl FnOnce(&[&StyleDeclaration]) -> R) -> R {
        let mut resolved: SmallVec<[(Cow<'_, StyleDeclaration>, Rank); 4]> = SmallVec::new();
        self.resolve_ranked(&mut |declaration, rank| resolved.push((declaration, rank)));

        // With one priority, every declaration wins.
        if resolved.iter().all(|(_, rank)| rank.is_empty()) {
            let winners: SmallVec<[&StyleDeclaration; 4]> = resolved
                .iter()
                .map(|(declaration, _)| declaration.as_ref())
                .collect();
            return f(&winners);
        }

        let mut winning = SmallVec::<[(&str, &Rank); 4]>::new();
        for (declaration, rank) in &resolved {
            let property = declaration.property_name();
            match winning
                .iter_mut()
                .find(|(candidate, _)| *candidate == property)
            {
                Some((_, current)) => *current = (*current).min(rank),
                None => winning.push((property, rank)),
            }
        }
        let mut winners: SmallVec<[(&StyleDeclaration, &Rank); 4]> = resolved
            .iter()
            .filter(|(declaration, rank)| {
                winning.iter().any(|(property, winning_rank)| {
                    *property == declaration.property_name() && *winning_rank == rank
                })
            })
            .map(|(declaration, rank)| (declaration.as_ref(), rank))
            .collect();
        winners.sort_by(|(_, a), (_, b)| b.cmp(a));
        let winners: SmallVec<[&StyleDeclaration; 4]> = winners
            .into_iter()
            .map(|(declaration, _)| declaration)
            .collect();
        f(&winners)
    }

    /// Append the current declarations to `buf` as `property:value;` pairs.
    ///
    /// Absent declarations and declarations hidden by a layer of higher priority are left out.
    /// Reading a reactive declaration subscribes the current reactive observer.
    pub(crate) fn write_style_string(&self, buf: &mut String) {
        self.with_winning(|winners| {
            for declaration in winners {
                declaration.write_to(buf);
            }
        });
    }

    /// Return the current declarations as `property:value;` pairs.
    ///
    /// Reading a reactive declaration subscribes the current reactive observer. To render an
    /// element, write `style=styles` instead, which updates only what changed.
    #[must_use]
    pub fn to_style_string(&self) -> String {
        let mut s = String::new();
        self.write_style_string(&mut s);
        s
    }

    /// Return whether any declaration is reactive. A style without reactive declarations
    /// renders once and never subscribes to signals.
    pub(crate) const fn is_reactive(&self) -> bool {
        self.styles.has_reactive
    }

    /// Return whether no entry can ever have a declaration.
    pub(crate) fn is_empty(&self) -> bool {
        self.styles.entries.is_empty()
    }
}

impl FromStr for Styles {
    type Err = InvalidUncheckedDeclaration;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// A single declaration converts into `Styles`, so a `#[prop(into)] styles: Styles` prop accepts
/// `styles=PADDING.declare(px(16))`.
impl From<typed_css::Declaration> for Styles {
    fn from(declaration: typed_css::Declaration) -> Self {
        Self::new().with(declaration)
    }
}

/// A single unchecked declaration converts into `Styles`.
impl From<UncheckedDeclaration> for Styles {
    fn from(declaration: UncheckedDeclaration) -> Self {
        Self::new().with(declaration)
    }
}

/// A [`StyleEntry`] converts into `Styles`, so a prop also accepts a closure wrapped in
/// [`StyleEntry::new`].
impl From<StyleEntry> for Styles {
    fn from(entry: StyleEntry) -> Self {
        Self::new().with(entry)
    }
}

/// An `Option` converts into `Styles`. `None` converts into an empty `Styles`.
impl<T> From<Option<T>> for Styles
where
    T: Into<StyleEntry>,
{
    fn from(entry: Option<T>) -> Self {
        Self::new().with(entry)
    }
}

/// An array of declarations, `Styles`, entries, or `Option`s of them converts into `Styles`.
impl<T, const N: usize> From<[T; N]> for Styles
where
    T: Into<StyleEntry>,
{
    fn from(entries: [T; N]) -> Self {
        entries.into_iter().collect()
    }
}

impl<T> FromIterator<T> for Styles
where
    T: Into<StyleEntry>,
{
    fn from_iter<I: IntoIterator<Item = T>>(entries: I) -> Self {
        let mut styles = Self::new();
        styles.extend(entries);
        styles
    }
}

impl<T> Extend<T> for Styles
where
    T: Into<StyleEntry>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, entries: I) {
        for entry in entries {
            self.styles.push(entry.into(), 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    use assertr::prelude::*;

    use crate::{InvalidUncheckedReason, StyleEntry, Styles, UncheckedDeclaration};

    fn toggled(
        flag: &Arc<AtomicBool>,
        value: &'static str,
    ) -> impl Fn() -> Option<UncheckedDeclaration> + Send + Sync + 'static {
        let flag = Arc::clone(flag);
        move || {
            flag.load(Ordering::Relaxed)
                .then(|| UncheckedDeclaration::new("color", value))
        }
    }

    mod construction {
        use super::*;

        #[test]
        fn new_renders_nothing() {
            assert_that!(Styles::new().to_style_string()).is_equal_to("");
        }

        #[test]
        fn declarations_render_in_insertion_order() {
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("color", "green"))
                .with(UncheckedDeclaration::new("font-size", "14px"))
                .with_all([UncheckedDeclaration::new("display", "grid")]);
            assert_that!(styles.to_style_string())
                .is_equal_to("color:green;font-size:14px;display:grid;");
        }

        #[test]
        fn optional_declarations_skip_none() {
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("color", "red"))
                .with(None::<UncheckedDeclaration>)
                .with(Some(UncheckedDeclaration::new("margin", "10px")));
            assert_that!(styles.to_style_string()).is_equal_to("color:red;margin:10px;");
        }

        #[test]
        fn duplicates_render_in_order() {
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("height", "100vh"))
                .with(UncheckedDeclaration::new("height", "100dvh"));
            assert_that!(styles.to_style_string()).is_equal_to("height:100vh;height:100dvh;");
        }

        #[test]
        fn options_convert_into_styles() {
            let collected: Styles = [Some(UncheckedDeclaration::new("color", "red")), None]
                .into_iter()
                .collect();
            let converted: Styles = Some(UncheckedDeclaration::new("gap", "4px")).into();
            let absent: Styles = None::<UncheckedDeclaration>.into();
            assert_that!(collected.to_style_string()).is_equal_to("color:red;");
            assert_that!(converted.to_style_string()).is_equal_to("gap:4px;");
            assert_that!(absent.styles.entries.len()).is_equal_to(0);
        }

        #[test]
        fn nested_styles_keep_their_fallbacks_above_the_fallbacks_of_self() {
            let base = || {
                Styles::new()
                    .with(UncheckedDeclaration::new("color", "red"))
                    .with_fallback(UncheckedDeclaration::new("margin", "1px"))
            };
            let extra = || {
                Styles::new()
                    .with(UncheckedDeclaration::new("color", "blue"))
                    .with_fallback(UncheckedDeclaration::new("margin", "2px"))
            };
            assert_that!(base().with(extra()).to_style_string())
                .is_equal_to("margin:2px;color:red;color:blue;");

            // A nested `Styles` behaves like a closure that returns it.
            assert_that!(base().with(extra).to_style_string())
                .is_equal_to("margin:2px;color:red;color:blue;");
        }

        #[test]
        fn nesting_does_not_depend_on_the_order_of_calls() {
            let nested = || {
                Styles::new()
                    .with(UncheckedDeclaration::new("gap", "2px"))
                    .with_fallback(UncheckedDeclaration::new("margin", "2px"))
            };
            let fallback = || UncheckedDeclaration::new("margin", "9px");
            let first = Styles::new().with_fallback(fallback()).with(nested());
            let second = Styles::new().with(nested()).with_fallback(fallback());
            assert_that!(first.to_style_string()).is_equal_to("margin:2px;gap:2px;");
            assert_that!(second.to_style_string()).is_equal_to("margin:2px;gap:2px;");
        }

        #[test]
        fn empty_and_absent_styles_are_not_stored() {
            let styles = Styles::new()
                .with(Styles::new())
                .with(None::<Styles>)
                .with(Some(
                    Styles::new().with(UncheckedDeclaration::new("gap", "1px")),
                ));
            assert_that!(styles.styles.entries.len()).is_equal_to(1);
            assert_that!(styles.is_reactive()).is_false();
            assert_that!(styles.to_style_string()).is_equal_to("gap:1px;");
        }

        #[test]
        fn absent_static_declarations_are_not_stored() {
            let styles = Styles::new().with(None::<UncheckedDeclaration>);
            assert_that!(styles.styles.entries.len()).is_equal_to(0);
        }

        #[test]
        fn declarations_collect_into_styles() {
            let mut styles: Styles = ["grid", "flex"]
                .into_iter()
                .map(|display| UncheckedDeclaration::new("display", display))
                .collect();
            styles.extend([StyleEntry::new(Some(UncheckedDeclaration::new(
                "gap", "4px",
            )))]);
            assert_that!(styles.to_style_string())
                .is_equal_to("display:grid;display:flex;gap:4px;");
        }

        #[test]
        fn single_declarations_and_arrays_convert_into_styles() {
            let single: Styles = UncheckedDeclaration::new("color", "red").into();
            let array: Styles = [
                StyleEntry::new(UncheckedDeclaration::new("color", "blue")),
                StyleEntry::new(Some(UncheckedDeclaration::new("padding", "20px"))),
            ]
            .into();
            assert_that!(single.to_style_string()).is_equal_to("color:red;");
            assert_that!(array.to_style_string()).is_equal_to("color:blue;padding:20px;");
        }
    }

    mod parsing {
        use super::*;

        #[test]
        fn parse_skips_empty_declarations() {
            let styles = Styles::parse(";;color: red;;padding: 1rem;;").unwrap();
            assert_that!(styles.to_style_string()).is_equal_to("color:red;padding:1rem;");
        }

        #[test]
        fn parse_keeps_semicolons_inside_urls() {
            let styles: Styles = "background: url(data:image/png;base64,AA) no-repeat"
                .parse()
                .unwrap();
            assert_that!(styles.to_style_string())
                .is_equal_to("background:url(data:image/png;base64,AA) no-repeat;");
        }

        #[test]
        fn parse_propagates_declaration_errors() {
            let error = Styles::parse("color: red; invalid").unwrap_err();
            assert_that!(error.declaration()).is_equal_to("invalid");
            assert_that!(error.reason()).is_equal_to(InvalidUncheckedReason::MissingSeparator);
        }
    }

    mod fallback {
        use super::*;

        #[test]
        fn fallback_property_names_are_normalized_before_comparison() {
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("touch-action", "none"))
                .with_fallback(
                    Styles::new().with(UncheckedDeclaration::new("Touch-Action", "auto")),
                );
            assert_that!(styles.to_style_string()).is_equal_to("touch-action:none;");
        }

        #[test]
        fn reactive_higher_layer_toggles_the_fallback() {
            let flag = Arc::new(AtomicBool::new(true));
            let styles = Styles::new()
                .with(toggled(&flag, "orange"))
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("color", "gray")));

            assert_that!(styles.to_style_string()).is_equal_to("color:orange;");
            flag.store(false, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("color:gray;");
            flag.store(true, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("color:orange;");
        }

        #[test]
        fn same_layer_duplicates_render_together_above_the_fallback() {
            let flag = Arc::new(AtomicBool::new(true));
            let styles = Styles::new()
                .with(toggled(&flag, "orange"))
                .with(UncheckedDeclaration::new("color", "red"))
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("color", "gray")));

            assert_that!(styles.to_style_string()).is_equal_to("color:orange;color:red;");
            flag.store(false, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("color:red;");
        }

        #[test]
        fn lower_layers_render_first() {
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("padding-top", "0"))
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("padding", "8px")))
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("margin", "4px")));
            assert_that!(styles.to_style_string())
                .is_equal_to("margin:4px;padding:8px;padding-top:0;");
        }

        #[test]
        fn chained_fallbacks_keep_their_order() {
            let styles = Styles::new()
                .with(None::<UncheckedDeclaration>)
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("color", "gray")))
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("color", "blue")));
            assert_that!(styles.to_style_string()).is_equal_to("color:gray;");
        }

        #[test]
        fn later_normal_entries_shadow_an_earlier_fallback() {
            let styles = Styles::new()
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("color", "fallback")))
                .with(UncheckedDeclaration::new("color", "user"));
            assert_that!(styles.to_style_string()).is_equal_to("color:user;");
        }

        #[test]
        fn empty_layers_merge_cleanly() {
            let merged = Styles::new().with_fallback(Styles::new());
            assert_that!(merged.to_style_string()).is_equal_to("");
            assert_that!(merged.is_reactive()).is_false();

            let appended = Styles::new()
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("color", "red")));
            assert_that!(appended.to_style_string()).is_equal_to("color:red;");
        }
    }

    mod reactivity {
        use super::*;

        #[test]
        fn static_containers_are_not_reactive() {
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("color", "red"))
                .with(Some(UncheckedDeclaration::new("padding", "8px")))
                .with(None::<UncheckedDeclaration>);
            assert_that!(styles.is_reactive()).is_false();
        }

        #[test]
        fn closures_outlive_the_owner_that_created_them() {
            let owner = leptos::prelude::Owner::new();
            let styles =
                owner.with(|| Styles::new().with(|| UncheckedDeclaration::new("color", "red")));
            drop(owner);
            assert_that!(styles.to_style_string()).is_equal_to("color:red;");
        }

        #[test]
        fn a_closure_can_switch_a_group_of_declarations() {
            let compact = Arc::new(AtomicBool::new(true));
            let flag = Arc::clone(&compact);
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("display", "grid"))
                .with(move || {
                    if flag.load(Ordering::Relaxed) {
                        Styles::new()
                            .with(UncheckedDeclaration::new("gap", "2px"))
                            .with(UncheckedDeclaration::new("padding", "4px"))
                    } else {
                        Styles::new().with(UncheckedDeclaration::new("gap", "8px"))
                    }
                })
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("padding", "16px")));

            assert_that!(styles.is_reactive()).is_true();
            assert_that!(styles.to_style_string()).is_equal_to("display:grid;gap:2px;padding:4px;");
            compact.store(false, Ordering::Relaxed);
            assert_that!(styles.to_style_string())
                .is_equal_to("padding:16px;display:grid;gap:8px;");
        }

        #[test]
        fn a_fallback_inside_a_group_stays_below_the_layer_of_the_group() {
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("padding-top", "0"))
                .with(|| {
                    Styles::new()
                        .with(UncheckedDeclaration::new("color", "red"))
                        .with_fallback(
                            Styles::new().with(UncheckedDeclaration::new("padding", "16px")),
                        )
                })
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("margin", "4px")));
            assert_that!(styles.to_style_string())
                .is_equal_to("margin:4px;padding:16px;padding-top:0;color:red;");

            let hidden = Styles::new()
                .with(UncheckedDeclaration::new("color", "red"))
                .with(|| {
                    Styles::new().with_fallback(
                        Styles::new().with(UncheckedDeclaration::new("color", "blue")),
                    )
                });
            assert_that!(hidden.to_style_string()).is_equal_to("color:red;");
        }

        #[test]
        fn groups_keep_their_rank_inside_fallbacks_and_other_groups() {
            let in_fallback = Styles::new()
                .with(UncheckedDeclaration::new("color", "red"))
                .with_fallback(Styles::new().with(|| {
                    Styles::new()
                        .with(UncheckedDeclaration::new("gap", "1px"))
                        .with_fallback(
                            Styles::new().with(UncheckedDeclaration::new("color", "blue")),
                        )
                }))
                .with_fallback(Styles::new().with(UncheckedDeclaration::new("gap", "9px")));
            assert_that!(in_fallback.to_style_string()).is_equal_to("gap:1px;color:red;");

            let nested = Styles::new()
                .with(UncheckedDeclaration::new("margin-top", "0"))
                .with(|| {
                    Styles::new().with(|| {
                        Styles::new()
                            .with(UncheckedDeclaration::new("color", "red"))
                            .with_fallback(
                                Styles::new().with(UncheckedDeclaration::new("margin", "4px")),
                            )
                    })
                });
            assert_that!(nested.to_style_string())
                .is_equal_to("margin:4px;margin-top:0;color:red;");
        }

        #[test]
        fn a_nested_reactive_styles_makes_the_receiver_reactive() {
            let flag = Arc::new(AtomicBool::new(true));
            let nested = Styles::new().with(toggled(&flag, "red"));
            let styles = Styles::new()
                .with(UncheckedDeclaration::new("gap", "1px"))
                .with(nested);
            assert_that!(styles.is_reactive()).is_true();
            assert_that!(styles.to_style_string()).is_equal_to("gap:1px;color:red;");
            flag.store(false, Ordering::Relaxed);
            assert_that!(styles.to_style_string()).is_equal_to("gap:1px;");
        }

        #[test]
        fn closures_are_reactive() {
            let flag = Arc::new(AtomicBool::new(true));
            let optional = Styles::new().with(toggled(&flag, "red"));
            let always = Styles::new().with(|| UncheckedDeclaration::new("color", "red"));
            assert_that!(optional.is_reactive()).is_true();
            assert_that!(always.is_reactive()).is_true();
        }
    }
}
