# Stellar Token Faucet

A simple Soroban smart contract dApp that allows users to claim **1 XLM** from a faucet on Stellar Testnet.

## Project Description

This is a beginner-friendly dApp built on the Stellar blockchain using Soroban smart contracts. The Token Faucet allows users to claim exactly **1 XLM** one time per address from the faucet. It's designed as a learning project to understand Soroban contract development and Stellar blockchain basics.

## Project Vision

This project serves as a stepping stone for developers new to Soroban and Stellar development. It demonstrates:
- How to write a Soroban smart contract in Rust
- How to manage persistent storage (tracking claimed addresses)
- How to handle user authentication in smart contracts
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a minimal, working example that beginners can study, deploy, and extend.

## Features

- **One-time claim**: Each address can only claim XLM once
- **Simple**: No admin dashboard, no complex logic
- **Beginner-friendly**: Clear, readable Rust code for Soroban
- **On-chain tracking**: Claim status stored permanently on blockchain

## Contract Details

- **Network**: Stellar Testnet
- **Claim Amount**: 1 XLM (1,000,000 stroops)
- **Limit**: 1 claim per address (tracked on-chain)
- **Contract ID**: [CCNRXQFUOQMZBJ3YPQ2F4T6TESSQKFSJOAPMMMGG3P3PQ4F326B2LKCN](https://lab.stellar.org/r/testnet/contract/CCNRXQFUOQMZBJ3YPQ2F4T6TESSQKFSJOAPMMMGG3P3PQ4F326B2LKCN)

## Project Structure

```
.
├── contracts/
│   └── token_faucet/       # Main faucet contract
│       ├── src/
│       │   └── lib.rs       # Contract code
│       ├── Cargo.toml
│       └── Makefile
├── Cargo.toml               # Workspace config
└── README.md
```

## Setup & Build

### Prerequisites

- Rust (latest stable)
- Stellar CLI (`stellar`)
- Docker (for WASM build)

### Build Contract

```bash
cd contracts/token_faucet
make build
```

This generates the WASM file at `target/wasm32v1-none/release/token_faucet.wasm`.

### Deploy to Testnet

1. Fund your deployer account on Stellar Testnet [Stellar Lab](https://laboratory.stellar.org)

2. Deploy the contract:
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/token_faucet.wasm \
  --source <DEPLOYER_SECRET> \
  --network testnet
```

3. Initialize the contract:
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <DEPLOYER_SECRET> \
  --network testnet \
  -- init
```

4. Fund the contract with XLM for distribution:
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <DEPLOYER_SECRET> \
  --network testnet \
  -- fund
```

## Usage

### Check if an address has claimed
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <USER_SECRET> \
  --network testnet \
  -- has_claimed \
  --user <USER_ADDRESS>
```

### Claim XLM
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <USER_SECRET> \
  --network testnet \
  -- claim \
  --user <USER_ADDRESS>
```

## Contract Functions

| Function | Description |
|----------|-------------|
| `init()` | Initialize the contract (once) |
| `claim(user)` | Claim 1 XLM (one time per address) |
| `has_claimed(user)` | Check if address has already claimed |

## How It Works

1. **Admin funds contract**: The deployer sends XLM to the contract address
2. **User calls claim**: Contract verifies user hasn't claimed before
3. **Transfer**: 1 XLM is transferred from contract to user
4. **Record**: User address is marked as "claimed" permanently

## Future Scope

Potential improvements for this project:

1. **Custom Token Faucet**: Instead of native XLM, create a custom Stellar token (Soroban Token) that users can claim
2. **Admin Interface**: Add a simple frontend to let admin fund the faucet and monitor usage
3. **Rate Limiting**: Add time-based limits (e.g., 1 claim per day per address)
4. **Multiple Amounts**: Allow users to choose different claim amounts
5. **Frontend dApp**: Build a simple web interface using React or plain HTML/JS for easier user interaction
6. **Tokenomics**: Add a simple governance mechanism where token holders can vote on faucet parameters

## License

MIT
