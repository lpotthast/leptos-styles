pub mod styles;

use std::time::{Duration, Instant};

use browser_test::thirtyfour::{By, WebDriver};
use leptos_browser_test::{Report, ResultExt, bail};
use tokio::time::sleep;

const POLL_TIMEOUT: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Returns `true` while polling should continue, `false` once the deadline has passed.
struct PollDeadline {
    deadline: Instant,
}

impl PollDeadline {
    fn new() -> Self {
        Self {
            deadline: Instant::now() + POLL_TIMEOUT,
        }
    }

    fn expired(&self) -> bool {
        Instant::now() >= self.deadline
    }

    async fn sleep_step(&self) {
        sleep(POLL_INTERVAL).await;
    }
}

pub trait BaseActions {
    fn driver(&self) -> &WebDriver;
    fn base_url(&self) -> &str;

    async fn goto_path(&self, path: &str) -> Result<(), Report> {
        let url = format!("{}{path}", self.base_url());
        self.driver()
            .goto(&url)
            .await
            .context_with(|| format!("failed to go to {url}"))?;
        Ok(())
    }

    async fn click_element_with_id(&self, id: &str) -> Result<(), Report> {
        tracing::info!("Click element with id '{id}'.");
        let element = self
            .driver()
            .find(By::Id(id))
            .await
            .context_with(|| format!("failed to find #{id}"))?;
        element
            .click()
            .await
            .context_with(|| format!("failed to click #{id}"))?;
        Ok(())
    }

    async fn style_of(&self, id: &str) -> Result<String, Report> {
        Ok(self.style_attribute(id).await?.unwrap_or_default())
    }

    async fn style_attribute(&self, id: &str) -> Result<Option<String>, Report> {
        let element = self
            .driver()
            .find(By::Id(id))
            .await
            .context_with(|| format!("failed to find #{id}"))?;
        Ok(element
            .attr("style")
            .await
            .context_with(|| format!("failed to read style attribute from #{id}"))?)
    }

    async fn wait_for_style(&self, id: &str, expected: &str) -> Result<(), Report> {
        let poll = PollDeadline::new();
        let mut last_seen = String::new();
        let mut last_error: Option<Report> = None;

        while !poll.expired() {
            match self.style_of(id).await {
                Ok(actual) => {
                    if actual == expected {
                        return Ok(());
                    }
                    last_seen = actual;
                }
                Err(err) => {
                    last_error = Some(err);
                }
            }
            poll.sleep_step().await;
        }

        match last_error {
            Some(last_error) => Err(last_error
                .context(format!(
                    "timed out waiting for #{id} style {expected:?}; last seen {last_seen:?}"
                ))
                .into_dynamic()),
            None => {
                bail!("timed out waiting for #{id} style {expected:?}; last seen {last_seen:?}");
            }
        }
    }

    async fn wait_for_no_style_attribute(&self, id: &str) -> Result<(), Report> {
        let poll = PollDeadline::new();
        let mut last_seen: Option<String> = None;

        while !poll.expired() {
            match self.style_attribute(id).await? {
                None => return Ok(()),
                Some(value) => last_seen = Some(value),
            }
            poll.sleep_step().await;
        }

        bail!("timed out waiting for #{id} to drop its style attribute; last seen {last_seen:?}");
    }
}
