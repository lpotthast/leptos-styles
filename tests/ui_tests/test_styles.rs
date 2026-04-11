use std::time::Duration;

use thirtyfour::{TimeoutConfiguration, WebDriver};

use crate::{
    pages::{BaseActions, styles::StylesPage},
    ui_tests::UiTest,
};

pub struct StylesTests {}

#[async_trait::async_trait]
impl UiTest for StylesTests {
    fn name(&self) -> String {
        "styles_tests".to_string()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()> {
        let mut timeouts = TimeoutConfiguration::default();
        timeouts.set_implicit(Some(Duration::from_secs(3)));
        driver.update_timeouts(timeouts).await?;

        let page = StylesPage { driver, base_url };
        page.goto().await?;

        page.wait_for_style("static-target", "display:grid;color:red;")
            .await?;
        page.wait_for_style("hydrate-target", "display:grid;color:fresh;")
            .await?;
        page.wait_for_style("reactive-target", "display:grid;color:tomato;")
            .await?;
        page.wait_for_style(
            "normalized-target",
            "touch-action:none;--ThemeAccent:tomato;",
        )
        .await?;
        page.wait_for_style("merge-fallback-target", "color:orange;")
            .await?;

        page.click("set-royalblue").await?;
        page.wait_for_style("reactive-target", "display:grid;color:royalblue;")
            .await?;

        page.click("clear-color").await?;
        page.wait_for_style("reactive-target", "display:grid;")
            .await?;

        page.click("clear-merge-color").await?;
        page.wait_for_style("merge-fallback-target", "color:gray;")
            .await?;

        page.click("set-merge-orange").await?;
        page.wait_for_style("merge-fallback-target", "color:orange;")
            .await?;

        Ok(())
    }
}
