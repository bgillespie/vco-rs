# vco-rs

A Rust crate for interacting with VMware SD-WAN Orchestrators (VCO). It is intended to provide a client crate, but also a CLI tool (`vcoctl`) to interact with VCO from the command line.

## Architecture

The crate is broken down into three sub-crates:

### `api_v1`

Defines the data types used in API calls, both requests and responses. These use `serde` for serialization and deserialization.

### `client`

Handles authentication and calls to the API. Makes the results and calls nice for consumers, providing a unified front-end that hides the internals.

### `cli`

A CLI tool for interacting with VCO. At the moment this doesn't do much; I'm using it to try out client calls as they're written.

