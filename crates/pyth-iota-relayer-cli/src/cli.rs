//! CLI definitions. Single `pyth-iota-relayer` binary with two
//! sibling subcommands:
//!
//! - `start` runs the daemon (hybrid heartbeat + deviation triggers).
//! - `coins {show,list,split,merge,transfer}` manages the gas-coin
//!   pool the daemon spends from.
//!
//! Global flags select the network (testnet/mainnet — picks the right
//! contract addresses and the default RPC), the feeds YAML, and the
//! signing key. The key is a Bech32 `iotaprivkey1...` string;
//! `--key` reads it inline (prefer `IOTA_PRIVATE_KEY` so it stays out
//! of shell history).

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use iota_sdk_types::{Address, ObjectId};

#[derive(Parser, Debug)]
#[command(
    name = "pyth-iota-relayer",
    about = "Pyth → IOTA price relayer + gas-coin pool manager",
    max_term_width = 80
)]
pub struct Cli {
    /// Network selector: `testnet` or `mainnet`.
    #[arg(long, value_enum, default_value_t = Network::Testnet, global = true)]
    pub network: Network,

    /// Override the network's default GraphQL RPC URL.
    #[arg(long, global = true)]
    pub rpc: Option<String>,

    /// Path to the feeds YAML config.
    #[arg(long, default_value = "config.yaml", global = true)]
    pub feeds: PathBuf,

    /// Bech32 IOTA private key (`iotaprivkey1...`). Prefer setting
    /// `IOTA_PRIVATE_KEY` in the environment.
    #[arg(long, env = "IOTA_PRIVATE_KEY", hide_env_values = true, global = true)]
    pub key: Option<String>,

    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Run the relayer daemon.
    Start {
        /// Maximum feeds per on-chain update tx. The relayer chunks
        /// firing feeds into groups of this size and submits the
        /// chunks in parallel. Smaller batches are cheaper per-feed
        /// on IOTA's super-linear gas curve.
        #[arg(long, default_value_t = 3)]
        max_feeds_per_tx: usize,
        /// Target value (in IOTA) for each hot gas coin maintained by
        /// the daemon. The pool is sized so we can submit
        /// `ceil(feeds / max_feeds_per_tx)` chunks in parallel.
        #[arg(long, default_value_t = 1.0)]
        gas_coin_target: f64,
        /// Minimum IOTA below which a hot coin triggers a rebalance
        /// (merge all + re-split into `gas_coin_target`-sized
        /// pieces) at the end of the tick.
        #[arg(long, default_value_t = 0.5)]
        gas_coin_min: f64,
    },

    /// Manage the gas-coin pool the daemon spends from.
    Coins {
        #[command(subcommand)]
        cmd: CoinsCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum CoinsCmd {
    /// Pool summary: total, count, largest, smallest, dust count.
    Show {
        /// Dust threshold (nIOTA) used by the summary's `dust` line.
        #[arg(long, default_value_t = 10_000_000)]
        dust_below: u64,
    },

    /// One row per coin: id, balance, version.
    List,

    /// Split a coin into `count` outputs of `amount` nIOTA each. Use
    /// before `start` to pre-populate fee-sized coins.
    Split {
        /// nIOTA per output coin.
        #[arg(long)]
        amount: u64,
        /// How many output coins to produce.
        #[arg(long)]
        count: usize,
        /// Source coin. Default: the gas coin (works with 1-coin pools).
        #[arg(long)]
        source: Option<ObjectId>,
    },

    /// Merge coins below `--dust-below` into `--into`.
    Merge {
        /// Dust threshold (nIOTA).
        #[arg(long, default_value_t = 10_000_000)]
        dust_below: u64,
        /// Target coin. Default: the gas coin.
        #[arg(long)]
        into: Option<ObjectId>,
    },

    /// Send `amount` nIOTA to `--to`.
    Transfer {
        /// Recipient address.
        #[arg(long)]
        to: Address,
        /// nIOTA to send.
        #[arg(long)]
        amount: u64,
        /// Source coin. Default: the gas coin.
        #[arg(long)]
        source: Option<ObjectId>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Network {
    Testnet,
    Mainnet,
}

impl Network {
    pub fn as_str(self) -> &'static str {
        match self {
            Network::Testnet => "testnet",
            Network::Mainnet => "mainnet",
        }
    }
}
