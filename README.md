# Solana Moneybox

This project implements a simple moneybox program on the Solana blockchain, allowing users to store and manage SOL tokens. It uses the Anchor framework for Solana smart contract development.

## Overview

The Moneybox program allows users to:
- Initialize a moneybox account
- Deposit SOL tokens into the moneybox
- Withdraw SOL tokens from the moneybox

## Prerequisites

- Rust
- Solana CLI
- Node.js
- Yarn or npm

## Project Structure

- `src/lib.rs`: The main Rust file containing the Solana program logic.
- `tests/`: Contains the TypeScript tests for the program.
- `client.ts`: TypeScript client script to interact with the program.

## Getting Started

### Install Dependencies

Ensure you have Rust, Solana CLI, Node.js, and Yarn or npm installed.

```bash
# Rust and Solana CLI
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.8.2/install)"

# Node.js
curl -sL https://deb.nodesource.com/setup_14.x | sudo -E bash -
sudo apt-get install -y nodejs

# Yarn (recommended)
npm install --global yarn
