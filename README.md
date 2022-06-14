# SoDefi

A set of rust crates for working with DeFi protocols across the Solana ecosystem.

# Supported Protocols

* Orca
* Raydium
* Atrix

# Crates

## `so-defi-config`

Configuration parsing create that supports parsing the output from the various API's offered by the following protocols:

* Orca
* Raydium
* Atrix

Provides helpers for parsing the configuration api's available from Orca, and Raydium.

## `so-defi-atrix`

* For working with atrix this is the crate you want to import

## `so-defi-token-list`

* Parses the public tokenlist to pull names, and token mints.
* Useful for brute forcing the name of a market given it's coin/pc constituents

# Usage

For usage information please see the various unit tests included in each crate and module.