//! Integration tests for the checked `leptos-css` declaration boundary.

use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use assertr::prelude::*;
use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::{
        CssColor, CssColorName, CssCustomProperty, GlobalKeyword, LengthPercentageAuto,
        LengthPercentageCalculation, Margin, NonNegativeLengthPercentage, Padding, Size,
        css_custom_property, em, pct, px, rem, rgb, var, vh, vw,
    },
    property::{
        AllProperty, BackgroundColorProperty, ColorProperty, HeightProperty, MarginProperty,
        PaddingProperty, RightProperty, TopProperty, WidthProperty,
    },
};

css_custom_property!(ACCENT_COLOR: CssColor = "--AccentColor");

fn sized(value: leptos_styles::css::CssDimension) -> Size {
    Size::from(NonNegativeLengthPercentage::new(value))
}

#[test]
fn checked_property_value_pairs_render_canonical_css() {
    let styles = Styles::builder()
        .with(WidthProperty, sized(px(320)))
        .with(HeightProperty, sized(vh(40)))
        .with(PaddingProperty, Padding::all(em(1.5)))
        .with(
            MarginProperty,
            Margin::All(LengthPercentageAuto::from(rem(2))),
        )
        .with(RightProperty, LengthPercentageAuto::from(vw(10)))
        .build();

    assert_that!(styles.to_style_string())
        .is_equal_to("width:320px;height:40vh;padding:1.5em;margin:2rem;right:10vw;".to_string());
}

#[test]
fn one_color_grammar_builds_multiple_precisely_named_properties() {
    let styles = Styles::builder()
        .with(ColorProperty, rgb(255, 0, 0))
        .with(BackgroundColorProperty, rgb(255, 0, 0))
        .build();

    assert_that!(styles.to_style_string())
        .is_equal_to("color:rgb(255, 0, 0);background-color:rgb(255, 0, 0);".to_string());
}

#[test]
fn checked_calculation_stays_inside_the_width_grammar() {
    let width = Size::Calculation(LengthPercentageCalculation::new(pct(100) - px(20)));
    let styles = Styles::new().add(WidthProperty, width);

    assert_that!(styles.to_style_string()).is_equal_to("width:calc(100% - 20px);".to_string());
}

#[test]
fn checked_custom_properties_and_references_retain_their_color_grammar() {
    let missing = CssCustomProperty::<CssColor>::new("--MissingColor");
    let styles = Styles::builder()
        .with(ACCENT_COLOR, CssColor::Named(CssColorName::Fuchsia))
        .with(
            ColorProperty,
            var(&ACCENT_COLOR, CssColor::Named(CssColorName::Black)),
        )
        .with(
            BackgroundColorProperty,
            var(&missing, CssColor::Named(CssColorName::CurrentColor)),
        )
        .build();

    assert_that!(styles.to_style_string()).is_equal_to(
        "--AccentColor:fuchsia;color:var(--AccentColor, black);background-color:var(--MissingColor, currentcolor);"
            .to_string(),
    );
}

#[test]
fn optional_checked_values_render_only_when_present() {
    let some = Styles::new().add_optional(TopProperty, Some(LengthPercentageAuto::from(px(8))));
    assert_that!(some.to_style_string()).is_equal_to("top:8px;".to_string());

    let none = Styles::new().add_optional(TopProperty, None::<LengthPercentageAuto>);
    assert_that!(none.to_style_string()).is_equal_to(String::new());
}

#[test]
fn signal_carrying_exact_property_value_updates_reactively() {
    let owner = Owner::new();
    owner.with(|| {
        let (color, set_color) = signal(Some(CssColor::Named(CssColorName::Red)));
        let styles = Styles::new().add_optional(ColorProperty, color);

        assert_that!(styles.is_reactive()).is_true();
        assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());

        set_color.set(Some(CssColor::Named(CssColorName::Blue)));
        assert_that!(styles.to_style_string()).is_equal_to("color:blue;".to_string());

        set_color.set(None);
        assert_that!(styles.to_style_string()).is_equal_to(String::new());
    });
}

#[test]
fn always_present_reactive_values_support_signals_and_derived_closures() {
    let owner = Owner::new();
    owner.with(|| {
        let (color, set_color) = signal(rgb(255, 0, 0));
        let color_for_background = color;
        let styles = Styles::builder()
            .with_reactive(ColorProperty, color)
            .build()
            .add_reactive(BackgroundColorProperty, move || color_for_background.get());

        assert_that!(styles.is_reactive()).is_true();
        assert_that!(styles.to_style_string())
            .is_equal_to("color:rgb(255, 0, 0);background-color:rgb(255, 0, 0);".to_string());

        set_color.set(rgb(0, 0, 255));
        assert_that!(styles.to_style_string())
            .is_equal_to("color:rgb(0, 0, 255);background-color:rgb(0, 0, 255);".to_string());
    });
}

#[test]
fn optional_checked_closure_can_derive_a_value() {
    let enabled = Arc::new(AtomicBool::new(true));
    let enabled_for_style = Arc::clone(&enabled);
    let styles = Styles::new().add_optional(PaddingProperty, move || {
        enabled_for_style
            .load(Ordering::Relaxed)
            .then(|| Padding::all(px(16)))
    });

    assert_that!(styles.to_style_string()).is_equal_to("padding:16px;".to_string());
    enabled.store(false, Ordering::Relaxed);
    assert_that!(styles.to_style_string()).is_equal_to(String::new());
}

#[test]
fn heterogeneous_checked_declarations_can_be_stored_and_added_together() {
    let declarations = [
        WidthProperty.declare(sized(pct(100))),
        ColorProperty.declare(CssColor::Named(CssColorName::Fuchsia)),
    ];
    let styles: Styles = declarations.into();

    assert_that!(styles.to_style_string()).is_equal_to("width:100%;color:fuchsia;".to_string());
}

#[test]
fn reactive_complete_declaration_can_change_its_property() {
    let use_background = Arc::new(AtomicBool::new(false));
    let evaluations = Arc::new(AtomicUsize::new(0));
    let use_background_for_style = Arc::clone(&use_background);
    let evaluations_for_style = Arc::clone(&evaluations);
    let styles = Styles::new().add_optional_declaration(move || {
        evaluations_for_style.fetch_add(1, Ordering::Relaxed);
        let value = CssColor::Named(CssColorName::Red);
        Some(if use_background_for_style.load(Ordering::Relaxed) {
            BackgroundColorProperty.declare(value)
        } else {
            ColorProperty.declare(value)
        })
    });

    assert_that!(styles.to_style_string()).is_equal_to("color:red;".to_string());
    assert_that!(evaluations.load(Ordering::Relaxed)).is_equal_to(1);

    use_background.store(true, Ordering::Relaxed);
    assert_that!(styles.to_style_string()).is_equal_to("background-color:red;".to_string());
    assert_that!(evaluations.load(Ordering::Relaxed)).is_equal_to(2);
}

#[test]
fn merge_resolves_priority_from_each_declarations_current_property() {
    let higher = Styles::builder()
        .with(ColorProperty, CssColor::Named(CssColorName::Red))
        .build();
    let lower = Styles::builder()
        .with(ColorProperty, CssColor::Named(CssColorName::Blue))
        .with(PaddingProperty, Padding::all(px(8)))
        .build();

    assert_that!(higher.merge(lower).to_style_string())
        .is_equal_to("color:red;padding:8px;".to_string());
}

#[test]
fn css_wide_keywords_use_explicit_checked_paths() {
    let styles = Styles::builder()
        .with_global(PaddingProperty, GlobalKeyword::Inherit)
        .with(AllProperty, GlobalKeyword::RevertLayer)
        .build();

    assert_that!(styles.to_style_string())
        .is_equal_to("padding:inherit;all:revert-layer;".to_string());
}

#[test]
fn parsed_css_and_explicit_unchecked_entries_remain_available_as_escape_hatches() {
    let parsed = Styles::parse_css("display: grid; color: blue").expect("CSS should parse");
    let extended = parsed.add_unchecked("font-family", "system-ui");

    assert_that!(extended.to_style_string())
        .is_equal_to("display:grid;color:blue;font-family:system-ui;".to_string());
}
