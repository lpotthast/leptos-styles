use assertr::prelude::*;
use leptos::tachys::html::style::IntoStyle;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{CssValue, Style::*, Styles};

#[test]
fn test_static_tuple() {
    let styles: Styles = ("color", "red").into();
    assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());
}

#[test]
fn test_static_string_parsing() {
    let styles: Styles = "padding: 10px".into();
    assert_that!(styles.to_style_string()).is_equal_to("padding:10px;".to_string());
}

#[test]
fn test_static_string_parsing_with_semicolon() {
    let styles: Styles = "margin: 5px;".into();
    assert_that!(styles.to_style_string()).is_equal_to("margin:5px;".to_string());
}

#[test]
fn test_array_of_tuples() {
    let styles: Styles = [("color", "blue"), ("padding", "20px")].into();
    assert_that!(styles.to_style_string()).is_equal_to("color:blue;padding:20px;".to_string());
}

#[test]
fn test_builder_pattern() {
    let styles = Styles::builder()
        .with("margin", "10px")
        .with("display", "flex")
        .build();
    assert_that!(styles.to_style_string()).is_equal_to("margin:10px;display:flex;".to_string());
}

#[test]
fn test_add_method() {
    let styles = Styles::new().add("color", "green").add("font-size", "14px");
    assert_that!(styles.to_style_string()).is_equal_to("color:green;font-size:14px;".to_string());
}

#[test]
fn test_static_optional_some() {
    let styles: Styles = ("color", Some("red")).into();
    assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());
}

#[test]
fn test_static_optional_none() {
    let styles: Styles = ("color", None::<&str>).into();
    assert_that!(styles.to_style_string()).is_equal_to(String::new());
}

#[test]
fn test_mixed_static_and_optional() {
    let styles = Styles::builder()
        .with("color", "red")
        .with_optional("padding", None::<&str>)
        .with("margin", "10px")
        .build();
    assert_that!(styles.to_style_string()).is_equal_to("color:red;margin:10px;".to_string());
}

#[test]
fn test_string_ownership_variants() {
    let styles1: Styles = ("color".to_string(), "red".to_string()).into();
    assert_that!(styles1.to_style_string()).is_equal_to("color:red;".to_string());

    let styles2: Styles = ("color", "red".to_string()).into();
    assert_that!(styles2.to_style_string()).is_equal_to("color:red;".to_string());

    let styles3: Styles = ("color".to_string(), "red").into();
    assert_that!(styles3.to_style_string()).is_equal_to("color:red;".to_string());
}

#[test]
fn test_empty_styles() {
    let styles = Styles::new();
    assert_that!(styles.to_style_string()).is_equal_to(String::new());
}

#[test]
fn test_drilling_down() {
    let initial: Styles = ("color", "red").into();
    let extended = initial.add("margin", "5px");
    let final_styles = extended.add("padding", "10px");

    assert_that!(final_styles.to_style_string())
        .is_equal_to("color:red;margin:5px;padding:10px;".to_string());
}

#[test]
fn test_style_enum() {
    let styles: Styles = (BackgroundColor, "red").into();
    assert_that!(styles.to_style_string()).is_equal_to("background-color:red;".to_string());
}

#[test]
fn test_into_style_to_html() {
    let styles: Styles = [("color", "blue"), ("padding", "20px")].into();
    let mut html = String::new();
    IntoStyle::to_html(styles, &mut html);
    assert_that!(html).is_equal_to("color:blue;padding:20px;".to_string());
}

#[test]
fn test_into_style_to_html_empty() {
    let styles = Styles::new();
    let mut html = String::new();
    IntoStyle::to_html(styles, &mut html);
    assert_that!(html).is_equal_to(String::new());
}

#[test]
fn test_into_style_to_html_with_none_values() {
    let styles = Styles::builder()
        .with("color", "red")
        .with_optional("padding", None::<&str>)
        .with("margin", "10px")
        .build();
    let mut html = String::new();
    IntoStyle::to_html(styles, &mut html);
    assert_that!(html).is_equal_to("color:red;margin:10px;".to_string());
}

#[test]
fn test_css_value_with_style_enum() {
    use crate::css::pct;

    let styles = Styles::new()
        .add(Left, pct(50.0))
        .add(Width, CssValue::Str("100%".into()));
    assert_that!(styles.to_style_string()).is_equal_to("left:50%;width:100%;".to_string());
}

#[test]
fn test_css_value_with_closure() {
    use crate::css::pct;

    let styles = Styles::new().add(Left, move || pct(75.0));
    assert_that!(styles.to_style_string()).is_equal_to("left:75%;".to_string());
}

#[test]
fn test_properties_are_normalized_for_standard_css_names() {
    let styles = Styles::builder()
        .with(" Color ", "red")
        .with("TOUCH-action", "none")
        .build();

    assert_that!(styles.to_style_string()).is_equal_to("color:red;touch-action:none;".to_string());
}

#[test]
fn test_custom_properties_preserve_case() {
    let styles = Styles::new().add(" --ThemeAccent ", "tomato");
    assert_that!(styles.to_style_string()).is_equal_to("--ThemeAccent:tomato;".to_string());
}

#[test]
fn test_merge_treats_property_names_case_insensitively() {
    let hook_styles = Styles::new().add("touch-action", "none");
    let user_styles = Styles::new().add("Touch-Action", "auto");

    assert_that!(hook_styles.merge(user_styles).to_style_string())
        .is_equal_to("touch-action:none;".to_string());
}

#[test]
fn test_merge_allows_static_none_fallback() {
    let hook_styles = Styles::new().add_optional("color", None::<&str>);
    let user_styles = Styles::new().add("Color", "gray");

    assert_that!(hook_styles.merge(user_styles).to_style_string())
        .is_equal_to("color:gray;".to_string());
}

#[test]
fn test_merge_reactive_optional_fallback_toggles() {
    let use_hook_color = Arc::new(AtomicBool::new(true));
    let use_hook_color_for_style = Arc::clone(&use_hook_color);
    let hook_styles = Styles::new().add_optional("color", move || {
        if use_hook_color_for_style.load(Ordering::Relaxed) {
            Some("orange")
        } else {
            None
        }
    });
    let user_styles = Styles::new().add("Color", "gray");
    let styles = hook_styles.merge(user_styles);

    assert_that!(styles.to_style_string()).is_equal_to("color:orange;".to_string());

    use_hook_color.store(false, Ordering::Relaxed);
    assert_that!(styles.to_style_string()).is_equal_to("color:gray;".to_string());

    use_hook_color.store(true, Ordering::Relaxed);
    assert_that!(styles.to_style_string()).is_equal_to("color:orange;".to_string());
}

#[test]
fn test_merge_static_some_suppresses_fallback() {
    let hook_styles = Styles::new().add("color", "orange");
    let user_styles = Styles::new().add("Color", "gray");

    assert_that!(hook_styles.merge(user_styles).to_style_string())
        .is_equal_to("color:orange;".to_string());
}

#[test]
fn test_to_style_string_matches_ssr_output() {
    let styles = Styles::builder()
        .with("display", "grid")
        .with("color", "tomato")
        .build();
    let mut html = String::new();
    IntoStyle::to_html(styles.clone(), &mut html);

    assert_that!(styles.to_style_string()).is_equal_to(html);
}
