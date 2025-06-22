# Blueshift Pinocchio Vault

A minimal Solana escrow vault program built with the [Pinocchio](https://github.com/anza-xyz/pinocchio) framework.
This repository implements PDA-based escrow creation, SPL-token vault initialization, secure deposits, taker execution, and refunds—all in pure Rust, no Anchor or JS required.

Features
--------
- make: Initialize an escrow PDA and its token vault
- take: Execute a trade—transfer tokens from the vault to taker
- refund: Return tokens from vault back to maker
- Self-contained: Zero Anchor macros, zero JS/TS—just Rust & Solana BPF

Prerequisites
-------------
- Rust (stable toolchain)
- Solana CLI (v1.14+) with BPF toolchain (`cargo build-sbf`)
- `pinocchio` crate for on-chain program scaffolding

Repository Layout
----------------
.
├── programs/
│   └── blueshift_pinocchio_vault/    # On-chain Rust program
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                # entrypoint & dispatcher
│           ├── state.rs              # Escrow account definition
│           └── instructions/         # make.rs, take.rs, refund.rs + helpers.rs
├── Cargo.toml                        # Workspace manifest
└── README.md                         # This file

Building
--------
Compile the BPF library:

```
cargo build-sbf
```

The output `.so` will be generated under:

```
target/sbpf-solana-solana/release/libblueshift_pinocchio_vault.so
```

Deployment
----------
Deploy your program to a cluster:

```
solana program deploy \
  target/sbpf-solana-solana/release/libblueshift_pinocchio_vault.so \
  --program-id path/to/escrow-program-keypair.json
```

Usage
-----
Invoke instructions via the Solana CLI:

```
# Example: Initialize an escrow (make)
solana program invoke \
  --program-id <PROGRAM_ID> \
  --signer maker.json \
  --data <seed:U64><receive:U64><amount:U64> \
  --accounts maker,<ESCROW_PDA>,<MINT_A>,<MINT_B>,<MAKER_ATA_A>,<VAULT_ATA>,<SYS_PROGRAM>,<TOKEN_PROGRAM>
```

Replace with appropriate account addresses for **take** and **refund** commands.
