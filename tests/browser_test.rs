#![cfg(not(target_arch = "wasm32"))]

mod pages;
mod test_app;
mod ui_tests;

use std::sync::Once;

use anyhow::Result;
use chrome_for_testing_manager::{
    Channel, Chromedriver, PortRequest, Session, SessionError, VersionRequest,
};
use thirtyfour::{ChromeCapabilities, ChromiumLikeCapabilities};
use ui_tests::UiTest;

const DELAY_TEST_EXECUTION: bool = false;
static INIT_TRACING: Once = Once::new();

fn init_tracing() {
    INIT_TRACING.call_once(|| {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<()> {
    init_tracing();

    let frontend = test_app::start_frontend().await?;
    let base_url = frontend.base_url.clone();

    if DELAY_TEST_EXECUTION {
        tracing::info!("Continue with tests? y/n");
        let mut buf = String::new();
        loop {
            buf.clear();
            let input = std::io::stdin().read_line(&mut buf);
            if input.is_ok() {
                match buf.trim() {
                    "y" => break,
                    "n" => return Ok(()),
                    _ => {}
                }
            }
            if let Err(err) = input {
                tracing::error!("Error reading input: {err:?}");
                return Err(err.into());
            }
        }
    }

    let tests: Vec<Box<dyn UiTest>> = vec![Box::new(ui_tests::test_styles::StylesTests {})];

    tracing::info!("Starting webdriver...");
    let chromedriver =
        Chromedriver::run(VersionRequest::LatestIn(Channel::Stable), PortRequest::Any).await?;

    #[allow(clippy::redundant_closure_for_method_calls)]
    let test_result = async {
        for test in tests {
            chromedriver
                .with_custom_session(
                    |caps: &mut ChromeCapabilities| {
                        if std::env::var("BROWSER_TEST_VISIBLE").is_ok() {
                            caps.unset_headless()?;
                        }
                        Ok(())
                    },
                    async |session: &Session| {
                        tracing::info!("Executing test: {}", test.name());
                        test.run(session, &base_url)
                            .await
                            .map_err(|err| SessionError::Panic {
                                reason: err.to_string(),
                            })?;

                        Ok(())
                    },
                )
                .await?;
        }
        Ok::<(), anyhow::Error>(())
    }
    .await;

    chromedriver.terminate().await?;
    drop(frontend);
    test_result
}
