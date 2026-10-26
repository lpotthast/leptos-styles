//! Browser integration tests for the Leptos style attribute lifecycle.
#![cfg(not(target_arch = "wasm32"))]

mod pages;
mod ui_tests;

use std::time::Duration;

use browser_test::thirtyfour::ChromiumLikeCapabilities;
use browser_test::{
    BrowserTestRunner, BrowserTests, Cancellation, FailurePolicy, Pause, Timeouts, Visibility,
};
use leptos_browser_test::{LeptosTestAppConfig, Report};

#[tokio::test(flavor = "multi_thread")]
async fn browser_tests() -> Result<(), Report> {
    tracing_subscriber::fmt().init();

    let app = LeptosTestAppConfig::new("testing/test-app")
        .with_app_name("leptos-styles test app")
        .start()
        .await?;

    BrowserTestRunner::new(Cancellation::on_shutdown_signals())
        .with_chrome_capabilities(|caps| {
            // `--no-sandbox` disables Chrome's child-process sandboxing. User-mode tarball
            // extraction can't set the setuid-root bit on `chrome_sandbox` (the privileged helper
            // Chrome execs to install the sandbox), and CI kernels often also restrict
            // unprivileged user namespaces. Without either layer, Chrome exits before chromedriver
            // opens a session.
            caps.add_arg("--no-sandbox")?;

            // `--disable-dev-shm-usage` makes Chrome place its IPC shared-memory segments under
            // `/tmp` instead of `/dev/shm`. CI runners typically expose `/dev/shm` tiny tmpfs,
            // which Chrome can exhaust during session startup.
            caps.add_arg("--disable-dev-shm-usage")?;
            Ok(())
        })
        .with_failure_policy(FailurePolicy::RunAll)
        .with_visibility(Visibility::from_env()?.unwrap_or_default())
        .with_pause(Pause::from_env()?.unwrap_or_default())
        .with_timeouts(Timeouts::new().with_implicit_wait(Duration::from_secs(3)))
        .run(
            app.base_url(),
            BrowserTests::sequential().with(ui_tests::test_styles::StylesTests {}),
        )
        .await?;

    Ok(())
}
