use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use leptos_browser_test::Report;

use crate::pages::{BaseActions, styles::StylesPage};

pub struct StylesTests {}

#[async_trait]
impl BrowserTest<str> for StylesTests {
    fn name(&self) -> Cow<'_, str> {
        "styles_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = StylesPage { driver, base_url };
        page.goto().await?;

        assert_static_target(&page).await?;
        assert_hydrate_target(&page).await?;
        assert_reactive_target(&page).await?;
        assert_normalized_target(&page).await?;
        assert_merge_fallback_target(&page).await?;
        assert_typed_value_target(&page).await?;
        assert_checked_declaration_target(&page).await?;
        assert_drilling_target(&page).await?;
        assert_transition_target(&page).await?;
        assert_managed_target(&page).await?;
        assert_empty_target(&page).await?;
        assert_var_target(&page).await?;
        assert_calc_target(&page).await?;
        assert_units_target(&page).await?;
        assert_parse_css_target(&page).await?;

        Ok(())
    }
}

async fn assert_static_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("static-target", "display:grid;color:red;")
        .await
}

async fn assert_hydrate_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("hydrate-target", "display:grid;color:fresh;")
        .await
}

async fn assert_reactive_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("reactive-target", "display:grid;color:tomato;")
        .await?;

    page.click("set-royalblue").await?;
    page.wait_for_style("reactive-target", "display:grid;color:royalblue;")
        .await?;

    page.click("clear-color").await?;
    page.wait_for_style("reactive-target", "display:grid;")
        .await
}

async fn assert_normalized_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "normalized-target",
        "touch-action:none;--ThemeAccent:tomato;",
    )
    .await
}

async fn assert_merge_fallback_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("merge-fallback-target", "color:orange;")
        .await?;

    page.click("clear-merge-color").await?;
    page.wait_for_style("merge-fallback-target", "color:gray;")
        .await?;

    page.click("set-merge-orange").await?;
    page.wait_for_style("merge-fallback-target", "color:orange;")
        .await
}

async fn assert_typed_value_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("typed-value-target", "width:120px;color:red;")
        .await?;

    page.click("set-typed-blue").await?;
    page.wait_for_style("typed-value-target", "width:120px;color:blue;")
        .await?;

    page.click("set-typed-transparent").await?;
    page.wait_for_style("typed-value-target", "width:120px;color:transparent;")
        .await
}

async fn assert_checked_declaration_target(page: &StylesPage<'_>) -> Result<(), Report> {
    // The initial declaration is server-rendered and then hydrated in place.
    page.wait_for_style("checked-declaration-target", "color:rgb(255, 0, 0);")
        .await?;

    page.click("checked-value-update").await?;
    page.wait_for_style("checked-declaration-target", "color: rgb(0, 0, 255);")
        .await?;

    page.click("checked-property-update").await?;
    page.wait_for_style(
        "checked-declaration-target",
        "background-color: rgb(0, 128, 0);",
    )
    .await?;

    page.click("checked-custom-property").await?;
    page.wait_for_style(
        "checked-declaration-target",
        "--DirectAccent: rgb(255, 0, 255);",
    )
    .await?;

    page.click("checked-reset").await?;
    page.wait_for_style("checked-declaration-target", "").await
}

async fn assert_drilling_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("drilling-target", "color:purple;padding:5px;margin:10px;")
        .await
}

async fn assert_transition_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("transition-target", "display:grid;color:seagreen;")
        .await?;

    page.click("transition-clear").await?;
    page.wait_for_style("transition-target", "display:grid;")
        .await?;

    page.click("transition-set-color").await?;
    page.wait_for_style("transition-target", "display:grid;color:seagreen;")
        .await?;

    page.click("transition-freeze-static").await?;
    page.wait_for_style("transition-target", "display:grid;color:frozen;")
        .await?;

    page.click("transition-enable-reactive").await?;
    page.wait_for_style("transition-target", "display:grid;color:seagreen;")
        .await?;

    page.click("transition-clear").await?;
    page.wait_for_style("transition-target", "display:grid;")
        .await
}

async fn assert_managed_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("managed-target", "display:grid;color:indigo;")
        .await?;

    page.click("managed-mutate").await?;
    page.wait_for_style("managed-target", "color:black;background-color:gold;")
        .await?;

    // Re-asserting the same signal value should still trigger reconciliation,
    // since `set` always notifies subscribers.
    page.click("managed-set-same-color").await?;
    page.wait_for_style("managed-target", "display:grid;color:indigo;")
        .await?;

    page.click("managed-mutate").await?;
    page.wait_for_style("managed-target", "color:black;background-color:gold;")
        .await?;

    page.click("managed-set-crimson").await?;
    page.wait_for_style("managed-target", "display:grid;color:crimson;")
        .await?;

    page.click("managed-set-indigo").await?;
    page.wait_for_style("managed-target", "display:grid;color:indigo;")
        .await
}

async fn assert_empty_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("empty-target", "color:ephemeral;")
        .await?;

    page.click("empty-clear").await?;
    page.wait_for_no_style_attribute("empty-target").await?;

    page.click("empty-restore").await?;
    page.wait_for_style("empty-target", "color:ephemeral;")
        .await
}

async fn assert_var_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "var-target",
        "--AccentColor:fuchsia;color:var(--AccentColor, black);background-color:var(--MissingColor, currentcolor);",
    )
    .await
}

async fn assert_calc_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "calc-target",
        "width:calc(100% - 20px);height:calc(50vh + 1rem);",
    )
    .await
}

async fn assert_units_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "units-target",
        "width:50%;height:40vh;padding:1.5em;margin:2rem;right:10vw;",
    )
    .await
}

async fn assert_parse_css_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("parse-css-target", "color:rebeccapurple;padding:8px;")
        .await
}
