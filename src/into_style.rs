use std::{future::Future, sync::Arc};

use leptos::{
    attr::IntoAttributeValue,
    tachys::{
        html::{
            attribute::{Attribute, NamedAttributeKey, NextAttribute},
            style::IntoStyle,
        },
        renderer::{Rndr, dom::Element},
    },
};
use reactive_graph::effect::RenderEffect;

use crate::Styles;

impl IntoAttributeValue for Styles {
    type Output = Arc<dyn Fn() -> String + Send + Sync>;

    fn into_attribute_value(self) -> Self::Output {
        Arc::new(move || {
            let mut s = String::new();
            self.write_style_string(&mut s);
            s
        })
    }
}

/// State for the `IntoStyle` implementation of `Styles`.
///
/// A purely static `Styles` value is rendered as a plain `style` attribute and tracked through
/// `Static`. As soon as any reactive signal participates, the state moves to `Reactive` and is
/// driven by a `RenderEffect` that reuses two string buffers to avoid allocations on each tick.
///
/// Both the enum and its variants are `#[non_exhaustive]` so future variants or fields can be
/// added without breaking downstream code that pattern-matches this hidden state type.
#[doc(hidden)]
#[non_exhaustive]
pub enum StylesState {
    #[non_exhaustive]
    Static { el: Element, current: String },
    #[non_exhaustive]
    Reactive {
        el: Element,
        effect: RenderEffect<(String, String)>,
    },
}

fn current_style_attribute(el: &Element) -> String {
    el.get_attribute("style").unwrap_or_default()
}

fn set_style_attribute(el: &Element, value: &str) {
    if value.is_empty() {
        Rndr::remove_attribute(el, "style");
    } else {
        Rndr::set_attribute(el, "style", value);
    }
}

fn sync_rendered_style_attribute(el: &Element, rendered: &str) {
    if current_style_attribute(el) != rendered {
        set_style_attribute(el, rendered);
    }
}

fn sync_style_attribute(styles: &Styles, el: &Element, current: &mut String, scratch: &mut String) {
    scratch.clear();
    styles.write_style_string(scratch);

    if current_style_attribute(el) != *scratch {
        set_style_attribute(el, scratch);
    }

    std::mem::swap(current, scratch);
}

fn create_styles_effect(
    styles: Styles,
    el: Element,
    buffers: (String, String),
) -> RenderEffect<(String, String)> {
    RenderEffect::new_with_value(
        move |prev: Option<(String, String)>| {
            let (mut current, mut scratch) = prev.unwrap_or_default();
            sync_style_attribute(&styles, &el, &mut current, &mut scratch);
            (current, scratch)
        },
        Some(buffers),
    )
}

fn build_static_state(styles: &Styles, el: &Element) -> StylesState {
    let mut current = String::new();
    styles.write_style_string(&mut current);
    sync_rendered_style_attribute(el, &current);
    StylesState::Static {
        el: el.clone(),
        current,
    }
}

fn build_reactive_state(styles: Styles, el: &Element, buffers: (String, String)) -> StylesState {
    StylesState::Reactive {
        el: el.clone(),
        effect: create_styles_effect(styles, el.clone(), buffers),
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
        if self.is_reactive() {
            build_reactive_state(self, el, (String::new(), String::new()))
        } else {
            build_static_state(&self, el)
        }
    }

    fn build(self, el: &Element) -> Self::State {
        if self.is_reactive() {
            build_reactive_state(self, el, (String::new(), String::new()))
        } else {
            build_static_state(&self, el)
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        match state {
            StylesState::Static { el, current } => {
                if self.is_reactive() {
                    let el = el.clone();
                    let current = std::mem::take(current);
                    *state = build_reactive_state(self, &el, (current, String::new()));
                } else {
                    current.clear();
                    self.write_style_string(current);
                    sync_rendered_style_attribute(el, current);
                }
            }
            StylesState::Reactive { el, effect } => {
                let (current, scratch) = effect.take_value().unwrap_or_default();
                if self.is_reactive() {
                    let el = el.clone();
                    *state = build_reactive_state(self, &el, (current, scratch));
                } else {
                    let mut current = current;
                    current.clear();
                    self.write_style_string(&mut current);
                    let el = el.clone();
                    sync_rendered_style_attribute(&el, &current);
                    *state = StylesState::Static { el, current };
                }
            }
        }
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        self.touch_reactive_dependencies();
    }

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn reset(state: &mut Self::State) {
        match state {
            StylesState::Static { el, current } => {
                current.clear();
                Rndr::remove_attribute(el, "style");
            }
            StylesState::Reactive { el, effect } => {
                let mut current = effect
                    .take_value()
                    .map(|(current, _)| current)
                    .unwrap_or_default();
                current.clear();
                Rndr::remove_attribute(el, "style");
                let el = el.clone();
                *state = StylesState::Static { el, current };
            }
        }
    }
}

impl Attribute for Styles {
    const MIN_LENGTH: usize = 0;

    type AsyncOutput = Self;
    type State = StylesState;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(
        self,
        _buf: &mut String,
        _class: &mut String,
        style: &mut String,
        _inner_html: &mut String,
    ) {
        IntoStyle::to_html(self, style);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        IntoStyle::hydrate::<FROM_SERVER>(self, el)
    }

    fn build(self, el: &Element) -> Self::State {
        IntoStyle::build(self, el)
    }

    fn rebuild(self, state: &mut Self::State) {
        IntoStyle::rebuild(self, state);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        IntoStyle::dry_resolve(self);
    }

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send {
        std::future::ready(self)
    }

    fn keys(&self) -> Vec<NamedAttributeKey> {
        vec![NamedAttributeKey::Attribute("style".into())]
    }
}

impl NextAttribute for Styles {
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;
    use leptos::tachys::html::style::IntoStyle;

    use crate::Styles;

    #[test]
    fn to_html_renders_empty_styles_as_empty_string() {
        let styles = Styles::new();
        let mut buf = String::new();
        styles.to_html(&mut buf);
        assert_that!(buf).is_equal_to(String::new());
    }

    #[test]
    fn to_html_renders_active_entries_in_order() {
        let styles = Styles::builder()
            .with_unchecked("display", "grid")
            .with_unchecked("color", "tomato")
            .build();
        let mut buf = String::new();
        styles.to_html(&mut buf);
        assert_that!(buf).is_equal_to("display:grid;color:tomato;".to_string());
    }

    #[test]
    fn to_html_omits_optional_entries_resolving_to_none() {
        let styles = Styles::builder()
            .with_unchecked("display", "grid")
            .with_optional_unchecked("color", Option::<&'static str>::None)
            .with_unchecked("padding", "8px")
            .build();
        let mut buf = String::new();
        styles.to_html(&mut buf);
        assert_that!(buf).is_equal_to("display:grid;padding:8px;".to_string());
    }

    #[test]
    fn to_html_appends_to_existing_buffer() {
        let styles = Styles::builder().with_unchecked("color", "red").build();
        let mut buf = String::from("display:grid;");
        styles.to_html(&mut buf);
        assert_that!(buf).is_equal_to("display:grid;color:red;".to_string());
    }

    #[test]
    fn empty_styles_are_not_reactive() {
        let styles = Styles::new();
        assert_that!(styles.is_reactive()).is_false();
    }

    #[test]
    fn static_only_styles_are_not_reactive() {
        let styles = Styles::builder()
            .with_unchecked("display", "grid")
            .with_unchecked("color", "tomato")
            .build();
        assert_that!(styles.is_reactive()).is_false();
    }

    #[test]
    fn to_html_matches_to_style_string() {
        let styles = Styles::builder()
            .with_unchecked("display", "grid")
            .with_unchecked("color", "tomato")
            .build();
        let mut html = String::new();
        IntoStyle::to_html(styles.clone(), &mut html);

        assert_that!(styles.to_style_string()).is_equal_to(html);
    }
}
