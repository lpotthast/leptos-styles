use std::future::Future;

use leptos::{
    reactive::effect::RenderEffect,
    tachys::{
        html::style::IntoStyle,
        renderer::{
            Rndr,
            dom::{CssStyleDeclaration, Element},
        },
    },
};

use crate::{StyleEntry, Styles, UncheckedDeclaration, style_entry::StyleDeclaration};

/// The render state of `style=styles` on an element.
///
/// A `Styles` without reactive declarations renders once. A reactive `Styles` keeps a render
/// effect that updates the element when a signal changes.
pub struct StylesState {
    el: Element,
    render: Render,
}

enum Render {
    Static(Applied),
    Reactive(RenderEffect<Applied>),
}

/// The declarations that `Styles` last set on the element, in the order they were set.
#[derive(Default)]
struct Applied(Vec<StyleDeclaration>);

/// Return the index of the first declaration that changed, or `None` if the declarations did not
/// change.
fn first_change(old: &[StyleDeclaration], new: &[&StyleDeclaration]) -> Option<usize> {
    let unchanged = old
        .iter()
        .zip(new)
        .take_while(|(old, new)| old == *new)
        .count();
    (unchanged < old.len() || unchanged < new.len()).then_some(unchanged)
}

/// Remove the properties of `old` and `new` from `style`, and then set `new` in order.
fn rewrite(style: &CssStyleDeclaration, old: &[StyleDeclaration], new: &[&StyleDeclaration]) {
    for property in old
        .iter()
        .map(StyleDeclaration::property_name)
        .chain(new.iter().map(|declaration| declaration.property_name()))
    {
        Rndr::remove_css_property(style, property);
    }
    for declaration in new {
        Rndr::set_css_property(style, declaration.property_name(), declaration.value());
    }
}

/// Return whether the element shows a value for `property`.
fn shows(style: &CssStyleDeclaration, property: &str) -> bool {
    style
        .get_property_value(property)
        .is_ok_and(|value| !value.is_empty())
}

impl Applied {
    /// Set the properties that `Styles` manages on the element to `winners`.
    ///
    /// Nothing happens if the declarations did not change. Otherwise, the update starts at the
    /// first changed declaration. It removes the properties from there on and sets them again in
    /// order, which keeps the order of shorthands and longhands. A property whose value the
    /// browser rejects stays unset instead of keeping an old value. The properties before the
    /// first change, and properties that `Styles` never declared, such as those of `style:`
    /// directives, stay untouched.
    ///
    /// Removing a property can clear part of an unchanged declaration, such as a removed
    /// `padding-top` that clears an unchanged `padding`. Such a declaration then shows no value.
    /// In that case, the update writes every declaration again, so that the result matches
    /// server-side rendering. The update also writes every declaration if an unchanged
    /// declaration shows no value before it starts. The browser cannot write some shorthands as
    /// one value, such as a `padding: var(--space)` next to a `padding-top`, so damage to them
    /// would go unnoticed. The same rule restores a property that other code removed, and a
    /// property whose value the browser rejected.
    fn apply(&mut self, el: &Element, winners: &[&StyleDeclaration]) {
        let Some(mut start) = first_change(&self.0, winners) else {
            return;
        };
        let style = Rndr::style(el);
        let damaged = {
            let kept = &self.0[..start];
            let shows_kept = || {
                kept.iter()
                    .all(|declaration| shows(&style, declaration.property_name()))
            };
            if shows_kept() {
                rewrite(&style, &self.0[start..], &winners[start..]);
                !shows_kept()
            } else {
                true
            }
        };
        if damaged {
            rewrite(&style, &self.0, winners);
            start = 0;
        }
        if winners.is_empty() {
            remove_empty_style_attribute(el);
        }
        self.0.truncate(start);
        self.0.extend(
            winners[start..]
                .iter()
                .map(|&declaration| declaration.clone()),
        );
    }

    /// Take over the declarations that the server rendered. The element only changes if its
    /// `style` attribute does not contain the client's declarations.
    fn hydrate(&mut self, el: &Element, winners: &[&StyleDeclaration]) {
        let mut rendered = String::new();
        for declaration in winners {
            declaration.write_to(&mut rendered);
        }
        let attribute = el.get_attribute("style").unwrap_or_default();
        if contains_declarations(&attribute, &rendered) {
            self.0 = winners
                .iter()
                .map(|&declaration| declaration.clone())
                .collect();
        } else {
            self.apply(el, winners);
        }
    }

    /// Apply the current winners of `styles`, or take them over during hydration.
    fn update(&mut self, el: &Element, styles: &Styles, hydrating: bool) {
        styles.with_winning(|winners| {
            if hydrating {
                self.hydrate(el, winners);
            } else {
                self.apply(el, winners);
            }
        });
    }
}

/// Return whether `rendered` occurs in `attribute` at the start of a declaration.
///
/// Other code, such as a `style:` directive, can write into the same attribute, so `rendered` is
/// not always the whole attribute. It must start a declaration, so that `color:red;` is not found
/// inside `background-color:red;`.
fn contains_declarations(attribute: &str, rendered: &str) -> bool {
    rendered.is_empty()
        || attribute.match_indices(rendered).any(|(index, _)| {
            let before = attribute[..index].trim_end_matches([' ', '\t', '\n', '\r', '\x0c']);
            before.is_empty() || before.ends_with(';')
        })
}

fn remove_empty_style_attribute(el: &Element) {
    if el
        .get_attribute("style")
        .is_some_and(|style| style.trim().is_empty())
    {
        Rndr::remove_attribute(el, "style");
    }
}

impl StylesState {
    fn new(styles: Styles, el: &Element, mut applied: Applied, mut hydrating: bool) -> Self {
        let render = if styles.is_reactive() {
            let el = el.clone();
            Render::Reactive(RenderEffect::new_with_value(
                move |previous: Option<Applied>| {
                    let mut applied = previous.unwrap_or_default();
                    applied.update(&el, &styles, std::mem::take(&mut hydrating));
                    applied
                },
                Some(applied),
            ))
        } else {
            applied.update(el, &styles, hydrating);
            Render::Static(applied)
        };
        Self {
            el: el.clone(),
            render,
        }
    }

    fn take_applied(&mut self) -> Applied {
        match std::mem::replace(&mut self.render, Render::Static(Applied::default())) {
            Render::Static(applied) => applied,
            Render::Reactive(effect) => effect.take_value().unwrap_or_default(),
        }
    }
}

impl IntoStyle for Styles {
    type AsyncOutput = Self;
    type State = StylesState;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn to_html(self, style: &mut String) {
        self.write_style_string(style);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        StylesState::new(self, el, Applied::default(), true)
    }

    fn build(self, el: &Element) -> Self::State {
        StylesState::new(self, el, Applied::default(), false)
    }

    fn rebuild(self, state: &mut Self::State) {
        let applied = state.take_applied();
        let el = state.el.clone();
        *state = StylesState::new(self, &el, applied, false);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        self.with_winning(|_| ());
    }

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> {
        std::future::ready(self)
    }

    fn reset(state: &mut Self::State) {
        let mut applied = state.take_applied();
        applied.apply(&state.el, &[]);
        state.render = Render::Static(applied);
    }
}

/// Implement `IntoStyle` for a type that converts into `Styles`, by rendering that `Styles`. So
/// `style=value` behaves exactly like `style=Styles::from(value)`.
macro_rules! into_style_through_styles {
    ($($ty:ty),*) => {$(
        impl IntoStyle for $ty {
            type AsyncOutput = Self;
            type State = StylesState;
            type Cloneable = Self;
            type CloneableOwned = Self;

            fn to_html(self, style: &mut String) {
                Styles::from(self).to_html(style);
            }

            fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
                Styles::from(self).hydrate::<FROM_SERVER>(el)
            }

            fn build(self, el: &Element) -> Self::State {
                Styles::from(self).build(el)
            }

            fn rebuild(self, state: &mut Self::State) {
                Styles::from(self).rebuild(state);
            }

            fn into_cloneable(self) -> Self::Cloneable {
                self
            }

            fn into_cloneable_owned(self) -> Self::CloneableOwned {
                self
            }

            fn dry_resolve(&mut self) {
                Styles::from(self.clone()).dry_resolve();
            }

            fn resolve(self) -> impl Future<Output = Self::AsyncOutput> {
                std::future::ready(self)
            }

            fn reset(state: &mut Self::State) {
                Styles::reset(state);
            }
        }
    )*};
}

into_style_through_styles!(StyleEntry, UncheckedDeclaration);

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use leptos::tachys::html::style::IntoStyle;

    use super::{contains_declarations, first_change};
    use crate::{Styles, UncheckedDeclaration, style_entry::StyleDeclaration};

    fn declarations(pairs: &[(&'static str, &'static str)]) -> Vec<StyleDeclaration> {
        pairs
            .iter()
            .map(|&(property, value)| {
                StyleDeclaration::Unchecked(UncheckedDeclaration::new(property, value))
            })
            .collect()
    }

    fn start(
        old: &[(&'static str, &'static str)],
        new: &[(&'static str, &'static str)],
    ) -> Option<usize> {
        let old = declarations(old);
        let new = declarations(new);
        let new: Vec<&StyleDeclaration> = new.iter().collect();
        first_change(&old, &new)
    }

    #[test]
    fn server_declarations_are_found_only_at_a_declaration_start() {
        assert_that!(contains_declarations("color:red;", "color:red;")).is_true();
        assert_that!(contains_declarations(
            "opacity:0.5;color:red;",
            "color:red;"
        ))
        .is_true();
        assert_that!(contains_declarations(
            "opacity:0.5; color:red;",
            "color:red;"
        ))
        .is_true();
        assert_that!(contains_declarations("anything", "")).is_true();
        assert_that!(contains_declarations("background-color:red;", "color:red;")).is_false();
        assert_that!(contains_declarations("color:blue;", "color:red;")).is_false();
    }

    #[test]
    fn updates_start_at_the_first_change() {
        let display = ("display", "grid");
        assert_that!(start(
            &[display, ("color", "red")],
            &[display, ("color", "red")]
        ))
        .is_none();
        assert_that!(start(&[display], &[display, ("color", "red")])).is_equal_to(Some(1));
        assert_that!(start(&[display, ("color", "red")], &[display])).is_equal_to(Some(1));
        assert_that!(start(
            &[display, ("color", "red")],
            &[display, ("color", "blue")]
        ))
        .is_equal_to(Some(1));
        assert_that!(start(&[("color", "red")], &[display, ("color", "red")])).is_equal_to(Some(0));
    }

    #[test]
    fn to_html_renders_present_entries_in_order() {
        let styles = Styles::new()
            .with(UncheckedDeclaration::new("display", "grid"))
            .with(None::<crate::UncheckedDeclaration>)
            .with(UncheckedDeclaration::new("padding", "8px"));
        let mut buf = String::from("color:red;");
        styles.clone().to_html(&mut buf);

        assert_that!(buf).is_equal_to("color:red;display:grid;padding:8px;");
        assert_that!(styles.to_style_string()).is_equal_to("display:grid;padding:8px;");
    }

    #[test]
    fn to_html_renders_empty_styles_as_empty_string() {
        let mut buf = String::new();
        Styles::new().to_html(&mut buf);
        assert_that!(buf).is_equal_to("");
    }
}
