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
/// Uses `RenderEffect` to automatically track reactive dependencies within the styles
/// and update the DOM when any tracked signal changes.
pub struct StylesState {
    effect: RenderEffect<(Element, String, String)>,
}

fn current_style_attribute(el: &Element) -> String {
    el.get_attribute("style").unwrap_or_default()
}

fn sync_style_attribute(styles: &Styles, el: &Element, current: &mut String, scratch: &mut String) {
    scratch.clear();
    styles.write_style_string(scratch);

    if scratch != current {
        if scratch.is_empty() {
            Rndr::remove_attribute(el, "style");
        } else {
            Rndr::set_attribute(el, "style", scratch);
        }
    }

    std::mem::swap(current, scratch);
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
        let el = el.clone();
        StylesState {
            effect: RenderEffect::new(move |prev: Option<(Element, String, String)>| {
                let (el, mut current, mut scratch) = prev
                    .unwrap_or_else(|| (el.clone(), current_style_attribute(&el), String::new()));
                sync_style_attribute(&self, &el, &mut current, &mut scratch);
                (el, current, scratch)
            }),
        }
    }

    fn build(self, el: &Element) -> Self::State {
        let el = el.clone();
        StylesState {
            effect: RenderEffect::new(move |prev: Option<(Element, String, String)>| {
                if let Some((el, mut current, mut scratch)) = prev {
                    sync_style_attribute(&self, &el, &mut current, &mut scratch);
                    (el, current, scratch)
                } else {
                    let mut current = current_style_attribute(&el);
                    let mut scratch = String::new();
                    sync_style_attribute(&self, &el, &mut current, &mut scratch);
                    (el.clone(), current, scratch)
                }
            }),
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        let prev_value = state.effect.take_value();
        if let Some((el, current, scratch)) = prev_value {
            state.effect = RenderEffect::new_with_value(
                move |prev: Option<(Element, String, String)>| {
                    if let Some((el, mut current, mut scratch)) = prev {
                        sync_style_attribute(&self, &el, &mut current, &mut scratch);
                        (el, current, scratch)
                    } else {
                        unreachable!("rebuild should always have previous value")
                    }
                },
                Some((el, current, scratch)),
            );
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
        if let Some((el, _, _)) = state.effect.take_value() {
            Rndr::remove_attribute(&el, "style");
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
