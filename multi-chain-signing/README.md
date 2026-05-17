# Universal Social-Derived Wallet SDK (BTC + CKB)

## Overview

This project is a Rust-based wallet infrastructure SDK that generates deterministic Bitcoin and CKB wallets from a verified social identity (Google, Twitter, GitHub, email-based identity, etc.).

The goal is not to build a UI-heavy wallet app, but to deeply demonstrate how wallet systems work under the hood:

- deterministic key generation
- HD wallet derivation (BIP32/BIP39)
- Bitcoin transaction construction (UTXO model)
- CKB transaction construction (cell model)
- signing and broadcasting logic
- cross-chain wallet abstraction via a unified SDK

The system behaves like a CLI-driven “wallet engine” where a user can log in with a social identity and immediately derive reproducible wallets and transact on-chain.

---

## Core Idea

A user provides a verified social identity (ideally, but will be mocked and pre-authenticated in this demo). From this identity, the SDK deterministically generates:

- a root entropy seed
- a BIP32 master key (tprv/xprv equivalent)
- derived wallets for:
  - Bitcoin (testnet/mainnet)
  - CKB

The same identity always produces the same wallets.

---

## Social Authentication (Demo Simplification)

In this implementation, OAuth is **not implemented in full**.

Instead, the SDK assumes a verified identity is already provided.

### Input Example

```bash
wallet-cli login \
  --provider google \
  --id alice@gmail.com
````

### Assumptions

* The identity is already authenticated externally
* The SDK trusts the input identity for demo purposes
* No OAuth flow is implemented in the CLI
* The identity is treated as a canonical string

### Canonical Identity Format

```txt
google:alice@gmail.com
twitter:alice123
github:alice-dev
```

This canonical identity becomes the base for deterministic wallet generation.

---

## Deterministic Wallet Generation Flow

### 1. Identity → Entropy

The canonical identity is combined with a server-side secret to generate deterministic entropy:

```rust
entropy = HMAC_SHA256(server_secret, canonical_identity)
```

This ensures:

* Same identity always produces same entropy
* Prevents public derivation from identity alone
* Adds security via secret salt

---

### 2. Entropy → BIP39 Seed

The entropy is converted into a BIP39 mnemonic or directly into seed bytes:

* Optional mnemonic generation for debugging/demo
* Seed used for HD wallet derivation

---

### 3. Seed → BIP32 Root Key

The seed generates a master extended private key:

* `xprv` (Bitcoin mainnet)
* `tprv` (Bitcoin testnet)

This is the root of all derived wallets.

---

### 4. Chain-Specific Derivation

#### Bitcoin Wallet

Uses BIP44/BIP84 derivation paths:

```txt
m/84'/1'/0'/0/0  (testnet example)
```

Generates:

* Bitcoin address (bech32)
* private/public key pair
* UTXO-compatible signing key

---

#### CKB Wallet

Uses CKB lock script derivation patterns:

```txt
m/44'/309'/0'/0/0
```

Generates:

* lock script
* CKB address
* signing key for CKB transactions

---

## Wallet Abstraction Layer

The SDK exposes a unified wallet interface:

```rust
struct Wallet {
    btc: BitcoinWallet,
    ckb: CkbWallet,
}
```

### CLI Access

```bash
wallet-cli wallets list
wallet-cli address btc
wallet-cli address ckb
```

---

## Chain Execution Model

### Bitcoin (UTXO Model)

The SDK handles:

1. UTXO fetching
2. Input selection
3. Fee estimation
4. PSBT construction
5. Signing using derived key
6. Transaction broadcast

---

### CKB (Cell Model)

The SDK handles:

1. Cell collection via indexer
2. Transaction skeleton creation
3. Capacity balancing
4. Witness construction
5. Signing via lock script key
6. Transaction submission

---

## Signing Engine

A unified signing abstraction is used across chains:

```rust
trait ChainSigner {
    fn sign_transaction(&self, tx: Transaction) -> SignedTransaction;
}
```

Implementations:

* BitcoinSigner (PSBT-based signing)
* CkbSigner (witness-based signing)

This allows the SDK to treat all chains uniformly at the interface level.

---

## CLI Design

The system is fully CLI-driven to emphasize infrastructure design:

### Identity

```bash
wallet-cli login --provider google --id alice@gmail.com
```

### Wallet Derivation

```bash
wallet-cli derive --chain btc
wallet-cli derive --chain ckb
```

### Balance & State

```bash
wallet-cli balance --chain btc
wallet-cli utxos --chain btc
wallet-cli cells --chain ckb
```

### Transactions

```bash
wallet-cli send --chain btc --to tb1... --amount 10000
```

---

## Key Design Principles

### 1. Determinism

Same identity always produces same wallet:

* reproducible derivation
* no external state required for wallet generation

---

### 2. Chain Abstraction

Bitcoin and CKB are handled through separate adapters but exposed via a unified API.

---

### 3. Minimal Trust Assumptions

* identity is assumed verified in demo
* no full OAuth system implemented
* no UI dependency
* CLI acts as the only interface

---

### 4. Explicit Wallet Control

The root key is always required for signing transactions.

This ensures:

* full control remains with master identity
* deterministic wallets are reproducible
* no hidden custody logic

---

## What This Project Demonstrates

This project is designed to show deep understanding of:

* cryptographic key derivation (BIP32/BIP39)
* Bitcoin transaction architecture (UTXO model)
* CKB transaction model (cell system)
* wallet signing pipelines
* Rust systems design
* multi-chain abstraction engineering

---

## Summary

This SDK demonstrates how modern wallet infrastructure can be built from first principles:

1. Social identity → deterministic entropy
2. Entropy → HD wallet root
3. Root → multi-chain wallets
4. Wallets → transaction systems
5. Signing → chain execution

It is a minimal but powerful representation of how embedded wallet systems work internally across Bitcoin and CKB.

```
```
