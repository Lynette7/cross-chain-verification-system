# Cross-Chain Message Verifier

This cross-chain message verifier is a system designed to process, validate, and verify cross-chain messages securely using zero-knowledge (zkVM) technology. This system ensures message integrity, source/destination verification, and compatibility with blockchain-based smart contracts.

## Introduction

Cross-chain communication is essential in the multi-blockchain ecosystem. This system verifies messages between chains while maintaining security, privacy, and integrity. It employs zkVM proofs for message verification and integrates with Solidity smart contracts for on-chain validation.

## Features

- Zero-Knowledge Proofs: Ensures confidentiality and integrity of messages without exposing sensitive data.
- Cross-Chain Compatibility: Supports source and destination chain identification.
- Smart Contract Integration: Verifies proofs on-chain using a Solidity smart contract.
- Rust-Based zkVM: Built with Rust for performance and security.
- Fixture Generation: Provides testing utilities for generating proof fixtures for Solidity validation.

## Installation

1. Clone the Repository:

```bash
git clone https://github.com/Lynette7/cross-chain-verification-system.git
cd cross-chain-verification-system/verifier
```

2. Install Dependencies:
Ensure you have the following installed:

- Rust
- Foundry for Solidity testing

## Usage

1. Compile the SP1 program

- change directory to program:

``` bash
cd program
```

- Compile the program:

```bash
cargo prove build
```

2. Execute the program with the RISC-V runtime

```bash
RUST_LOG=info cargo run --release -- --execute --message "Hello, this is my first ZK application" --source-chain-id 1 --destination-chain-id 2
```

- If the execution of your program succeeds, then proof generation should succeed as well!

3. Generate zkVM Proofs

- Run Proof Generation:

```bash
RUST_LOG=info cargo run --release -- --prove --message "Hello, this is my first ZK application" --source-chain-id 1 --destination-chain-id 2
```

Parameters:

- --message: The cross-chain message to verify.
- --source-chain-id: ID of the source chain.
- --destination-chain-id: ID of the destination chain.
