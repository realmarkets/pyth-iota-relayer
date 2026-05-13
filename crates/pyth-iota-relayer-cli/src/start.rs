//! `start` — the relay daemon. Stubbed for now; the next milestones
//! fill in the Hermes client, the hybrid trigger state, and the
//! Wormhole + Pyth update PTB.

use anyhow::Result;
use iota_sdk_graphql_client::Client;
use iota_sdk_types::Address;
use owo_colors::OwoColorize;

use crate::cli::Network;
use crate::network::Contracts;

pub async fn run(
    _client: Client,
    sender: Address,
    network: Network,
    _key: Option<String>,
) -> Result<()> {
    let contracts = Contracts::for_network(network);
    println!(
        "{} relayer daemon not yet implemented.",
        "todo:".yellow().bold()
    );
    println!("       sender          {sender}");
    println!("       network         {}", network.as_str());
    println!("       pyth state      {}", contracts.pyth_state);
    println!("       wormhole state  {}", contracts.wormhole_state);
    Ok(())
}
