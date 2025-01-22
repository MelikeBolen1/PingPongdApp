# MultiversX PingPong dApp

A simple decentralized application built on MultiversX blockchain that allows users to deposit tokens (ping) and retrieve them after a set time period (pong).

## Overview

This dApp demonstrates basic blockchain interactions with the following features:
- Ping: Users can deposit 1 xEGLD
- Pong: Users can retrieve their deposit after a 10-minute waiting period
- One active ping per user at a time
- Smart contract written in Rust
- Frontend built with React and MultiversX SDK


## Prerequisites

- Node.js (v14 or later)
- Rust and Cargo
- MultiversX SDK (`mxpy`)
- A wallet with some xEGLD on MultiversX Devnet

## Setup and Installation

1. Clone the repository:
bash
git clone 
cd pingpongmultiversx

2. Install contract dependencies:
bash
cd contract
cargo build

3. Install frontend dependencies:
bash
cd dapp
yarn install


## Deployment

### Smart Contract

1. Build the contract:
bash
cd contract
sc-meta all build

2. Deploy to devnet:
bash
mxpy --verbose contract deploy \
--bytecode output/ping-pong.wasm \
--pem ../wallet/wallet-owner.pem \
--recall-nonce \
--gas-limit 60000000 \
--arguments 1000000000000000000 600 \
--chain D \
--proxy https://devnet-api.multiversx.com \
--send

### Frontend

1. Update the contract address in `dapp/src/config/config.devnet.ts`

2. Start the development server:
bash
cd dapp
yarn start:devnet

The application will be available at `http://localhost:3000`

## Usage

1. Connect your MultiversX wallet
2. Click "Ping" to deposit 1 xEGLD
3. Wait for 10 minutes
4. Click "Pong" to retrieve your deposit

## Testing

The smart contract includes basic tests that can be run with:
bash
cd contract
cargo test

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- MultiversX Team for providing the blockchain infrastructure
- MultiversX community for their support and resources
