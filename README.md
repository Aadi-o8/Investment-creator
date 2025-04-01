# Solana Investment Creator

The **Solana Investment Creator** is a decentralized application built on Solana, enabling groups to collaboratively create and manage investment funds. Developed in native Rust, this project leverages Solana’s high-performance blockchain to facilitate secure, transparent, and efficient group investing. Each fund operates independently with its own governance token, ensuring decision-making remains exclusive to its members.

---

## Project Overview

This platform empowers groups—such as friends, colleagues, or investment clubs—to pool resources and make collective investment decisions. Key features include:
- **Fund Creation**: Initialize a new investment fund by depositing SOL or other tokens.
- **Governance Tokens**: Receive tokens proportional to your deposit, granting voting rights within the fund.
- **Investment Proposals**: Submit detailed proposals specifying the amount, target asset, and decentralized exchange (DEX) for execution.
- **Voting Mechanism**: Use governance tokens to vote on proposals, with decisions determined by majority consensus.
- **Trade Execution**: Automatically execute approved investments on the designated DEX.

Each fund’s governance token is unique, preventing interference from external parties and ensuring autonomy.

---

## Core Features
- **Native Rust Implementation**: Built directly with the Solana Program Library (SPL) for maximum control and performance.
- **Custom Governance Tokens**: Issued via the SPL Token program, tailored to each fund.
- **Proposal System**: Structured process for proposing, voting on, and executing investments.
- **DEX Integration**: Designed to interface with Solana-based DEXs (e.g., Raydium, Serum) for seamless trades.
- **Security and Isolation**: Fund-specific tokens and accounts ensure decisions remain internal.

---

## Getting Started

### Prerequisites
- **Rust**: Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
- **Solana CLI**: Install with `sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"`.
- A Solana-compatible development environment (e.g., a local validator or devnet access).

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Aadi-o8/Investment-creator.git
   cd Investment-creator