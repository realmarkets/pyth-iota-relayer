//! Stateless hybrid trigger.
//!
//! "Should I publish?" is a pure function of:
//!
//! - the feed config (heartbeat / deviation / confidence thresholds),
//! - the *on-chain* price + publish_time (read each tick — never
//!   remembered locally),
//! - the latest Hermes parsed price.
//!
//! Heartbeat fires when `now - on_chain.publish_time > time_difference`.
//! Deviation fires when the Hermes price differs from on-chain by
//! more than `price_deviation` percent. Feeds whose Hermes confidence
//! ratio exceeds `confidence_ratio` percent are skipped regardless —
//! the upstream is too uncertain to publish.
//!
//! Reading chain state instead of remembering local pushes means
//! multiple relayer instances see the same view and converge on the
//! same decision; redundant relayers are safe.

use std::collections::HashMap;

use pyth_hermes_client::{FeedId, ParsedPrice};

use crate::feeds::FeedConfig;
use crate::on_chain::OnChainPrice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    Heartbeat,
    Deviation,
}

#[derive(Debug, Clone)]
pub struct Fire<'a> {
    pub cfg: &'a FeedConfig,
    pub hermes: &'a ParsedPrice,
    pub on_chain: OnChainPrice,
    pub reason: Reason,
}

pub fn fire<'a>(
    now_unix: u64,
    cfgs: &'a [FeedConfig],
    on_chain: &HashMap<FeedId, OnChainPrice>,
    hermes: &'a [ParsedPrice],
) -> Vec<Fire<'a>> {
    let mut out = Vec::new();
    for cfg in cfgs {
        let Some(hermes_price) = hermes.iter().find(|p| p.id == cfg.id) else {
            continue;
        };
        if exceeds_confidence(hermes_price, cfg.confidence_ratio) {
            continue;
        }
        let Some(&chain_price) = on_chain.get(&cfg.id) else {
            continue;
        };
        let reason = if now_unix.saturating_sub(chain_price.publish_time) > cfg.time_difference {
            Reason::Heartbeat
        } else if relative_diff_percent(chain_price, hermes_price) > cfg.price_deviation {
            Reason::Deviation
        } else {
            continue;
        };
        out.push(Fire {
            cfg,
            hermes: hermes_price,
            on_chain: chain_price,
            reason,
        });
    }
    out
}

fn exceeds_confidence(p: &ParsedPrice, ratio_pct: f64) -> bool {
    if p.price == 0 {
        return true;
    }
    let ratio = (p.conf as f64) * 100.0 / (p.price.unsigned_abs() as f64);
    ratio > ratio_pct
}

fn relative_diff_percent(on_chain: OnChainPrice, hermes: &ParsedPrice) -> f64 {
    let on_chain_f = (on_chain.price as f64) * 10f64.powi(on_chain.expo);
    let hermes_f = (hermes.price as f64) * 10f64.powi(hermes.expo);
    if on_chain_f == 0.0 {
        return f64::INFINITY;
    }
    ((hermes_f - on_chain_f).abs() / on_chain_f.abs()) * 100.0
}
