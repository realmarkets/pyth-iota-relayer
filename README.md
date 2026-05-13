# pyth-iota-relayer

A Pyth price-update relayer for the IOTA Rebased ledger. Pulls the
latest accumulator updates from Pyth's Hermes endpoint and posts them
on-chain whenever a feed exceeds its heartbeat or price-deviation
threshold. Ships with a `coins` subcommand for managing the relayer's
gas-coin pool.

## Build

```sh
cargo build --release -p pyth-iota-relayer-cli
# Binary: target/release/pyth-iota-relayer
```

## Configure

Feeds live in a YAML file — see [`configs/testnet.yaml`](configs/testnet.yaml)
for a working example. Each entry:

```yaml
- alias: BTC/USD
  id: e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43
  time_difference: 10       # heartbeat seconds — push if on-chain is older
  price_deviation: 0.5      # percent — push if Hermes differs by more
  confidence_ratio: 1       # percent — skip publish if Pyth conf/|price| is above this
```

The relayer's signing key is a Bech32 `iotaprivkey1…` string. Pass it
inline with `--key …` (handy for ad-hoc runs) or set
`IOTA_PRIVATE_KEY` in the environment (recommended — keeps it out of
shell history). Network selection (`--network testnet|mainnet`) picks
the right Pyth + Wormhole contract addresses and the default GraphQL
RPC; override the RPC with `--rpc <url>`.

## Run the relayer

```sh
export IOTA_PRIVATE_KEY=iotaprivkey1…
pyth-iota-relayer start --network testnet --feeds configs/testnet.yaml
```

The daemon ticks every second, fetches the latest Hermes snapshot and
the on-chain `PriceInfoObject`s, and submits one PTB per tick covering
the feeds whose on-chain state has gone stale or drifted past the
deviation threshold. It logs via `tracing`; set `RUST_LOG` (e.g.
`RUST_LOG=pyth_iota_relayer=debug`) to control verbosity. SIGINT or
SIGTERM trigger a clean shutdown.

Because the trigger decision is a pure function of on-chain state,
running multiple instances for redundancy is safe — they converge on
the same view and back off after one lands a tx.

## Manage the gas-coin pool

```sh
# Summary: total balance, coin count, largest/smallest, dust count.
pyth-iota-relayer coins show

# One row per coin.
pyth-iota-relayer coins list

# Pre-populate fee-sized coins before starting the daemon.
pyth-iota-relayer coins split --amount 100000000 --count 20

# Sweep dust into the gas coin.
pyth-iota-relayer coins merge --dust-below 10000000

# Move funds out.
pyth-iota-relayer coins transfer --to 0x… --amount 1000000000
```

All `coins` subcommands accept the same `--network` / `--rpc` /
`--feeds` / `--key` globals as `start`.

## License

Apache License 2.0 — see [`LICENSE`](LICENSE).
