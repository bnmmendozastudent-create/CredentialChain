# CredentialChain - Blockchain Verified Micro-Credentials

**One-line:** Soroban platform for employer-verified worker micro-credentials on Stellar blockchain.

## Problem
Migrant workers in SEA seeking Middle East jobs face:
- No formal certifications for on-the-job learned skills
- Forced to buy fake credentials for $500–$2,000
- Risk deportation if caught with fraudulent papers
- Miss 30–50% wage increases

## Solution
CredentialChain uses Soroban smart contracts:
1. Workers create micro-credentials
2. Employers verify & stake 500 USDC
3. Credentials become immutable on Stellar
4. Workers build portable portfolios
5. New employers scan & trust verified history

## Features
✅ Create credentials
✅ Employer verification with USDC staking
✅ Worker portfolio tracking
✅ Employer reputation scoring
✅ Portfolio monetization

## Contract Functions
- `initialize()` - Initialize
- `create_credential()` - Create credential
- `employer_verify_credential()` - Verify credential
- `worker_rate_employer()` - Rate employer
- `get_worker_portfolio()` - Get portfolio
- `add_to_portfolio()` - Add to portfolio
- `get_credential()` - Get credential details
- `get_employer()` - Get employer details
- `view_portfolio()` - View portfolio
- `get_credential_counter()` - Get count

## Tests
✅ All 5 tests pass

## Testnet Contract
- **Contract ID:** `CCPUHMRVEZ3RMTWO2SMITS4QZZXRCBXTFW4HU4GCCDODRSSONQJQKDB7`
- **Network:** Stellar Testnet

## Timeline
- **Week 1:** Design smart contract data keys, implement persistent storage logic, and complete all 5 required unit tests.
- **Week 2:** Deploy the optimized contract bytecode to the Stellar Testnet using Soroban Studio and design the web wallet connection interface.

## Stellar Features Used
- **Soroban Smart Contracts:** Powers the underlying lifecycle state machine tracking worker records, portfolio structures, and calculations for employer reputation tracking.
- **Stellar Core Assets (USDC Staking):** Enforces anti-fraud mechanics by programmatically lock-staking 500 USDC from verifying employer identities to guarantee authenticity.

## Vision and Purpose
To build an alternative peer-to-peer reputation and skill registry that bypassing expensive, predatory formal educational barriers. By shifting trust onto immutable blockchain records backed by economic stake, CredentialChain aims to deliver international labor mobility and higher wages safely to millions of undocumented blue-collar laborers globally.

## Prerequisites
- **Rust:** `rustc 1.75.0` or higher
- **Wasm Target:** `wasm32-unknown-unknown`
- **Soroban CLI:** Installed automatically or accessible via the Soroban Studio dashboard environment.

## How to Build
To compile the contract into highly optimized WebAssembly byte-code within your studio workspace terminal, run:
```bash
soroban contract build

Contract ID: CCPUHMRVEZ3RMTWO2SMITS4QZZXRCBXTFW4HU4GCCDODRSSONQJQKDB7
https://stellar.expert/explorer/testnet/contract/CCPUHMRVEZ3RMTWO2SMITS4QZZXRCBXTFW4HU4GCCDODRSSONQJQKDB7





