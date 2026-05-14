//! Small exponential-backoff retry helper for transient GraphQL
//! backend errors (gRPC "service unavailable", body-decode hiccups,
//! etc.).
//!
//! Call sites pass an async closure that produces a `Result<T>`; the
//! helper retries up to [`ATTEMPTS`] times with an exponential
//! backoff starting at [`BASE_BACKOFF`]. Each retry emits a `warn!`
//! including the operation label so operators can see how often the
//! backend is flaking.

use std::future::Future;
use std::time::Duration;

use anyhow::{anyhow, Result};
use tracing::warn;

const ATTEMPTS: u32 = 4;
const BASE_BACKOFF: Duration = Duration::from_millis(250);

pub async fn with_retry<F, Fut, T>(op: &str, mut f: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut last: Option<anyhow::Error> = None;
    for attempt in 1..=ATTEMPTS {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                if attempt < ATTEMPTS {
                    let backoff = BASE_BACKOFF * (1 << (attempt - 1));
                    warn!(
                        op,
                        attempt,
                        backoff_ms = backoff.as_millis() as u64,
                        error = %e,
                        "retrying",
                    );
                    tokio::time::sleep(backoff).await;
                }
                last = Some(e);
            }
        }
    }
    Err(last.unwrap_or_else(|| anyhow!("retry loop exited without an error")))
}
