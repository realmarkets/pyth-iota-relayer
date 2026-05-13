//! Common PTB submission flow used by every mutating subcommand
//! (`coins split`/`merge`/`transfer`, eventually `start`'s per-relay
//! tx).
//!
//! Owns the `submitting…` status line + the success printout so the
//! shape stays uniform across handlers. Maps `ExecuteError` into a
//! flat `anyhow::Error` for top-level reporting.

use anyhow::Result;
use iota_sdk_crypto::ed25519::Ed25519PrivateKey;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::execution_status::ExecutionStatus;
use move_bindgen_runtime::{ExecuteError, PtbBuilder};

use crate::fmt::print_ok;

/// Attach `client` + `signer`, enable auto-gas, submit, and print the
/// resulting tx digest on success. `op` is the human-readable verb
/// used in log lines (`"split"`, `"merge"`, …).
pub async fn execute(
    client: Client,
    signer: Ed25519PrivateKey,
    ptb: PtbBuilder,
    op: &str,
) -> Result<()> {
    println!("       submitting…");
    let ptb = ptb
        .with_client(client)
        .with_signer(signer)
        .with_auto_gas();
    match ptb.execute().await {
        Ok((effects, _cache)) => {
            let v1 = effects.as_v1();
            match v1.status() {
                ExecutionStatus::Success => {
                    print_ok(op, &v1.transaction_digest.to_string());
                    Ok(())
                }
                ExecutionStatus::Failure { error, command } => Err(anyhow::anyhow!(
                    "{op} aborted (digest={}, command={:?}): {:?}",
                    v1.transaction_digest,
                    command,
                    error,
                )),
                other => Err(anyhow::anyhow!("{op} unknown status: {other:?}")),
            }
        }
        Err(ExecuteError::Submit(e)) => Err(anyhow::anyhow!("{op} submit: {e}")),
        Err(e) => Err(anyhow::anyhow!("{op}: {e}")),
    }
}
