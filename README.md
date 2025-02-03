# AMM Anchor Project

This project implements an Automated Market Maker (AMM) using the Anchor framework on the Solana blockchain. The AMM uses a constant product formula for liquidity provision and token swaps.

## Features

1. Initialize liquidity pools
2. Deposit liquidity
3. Swap tokens

## Project Structure

- `instructions/`: Contains the main instruction logic
  - `initialize.rs`: Handles pool initialization
  - `deposit.rs`: Manages liquidity deposits
  - `swap.rs`: Executes token swaps
- `state/`: Defines the program's state
  - `config.rs`: Contains the `Config` struct for storing AMM state
- `lib.rs`: Entry point for the Anchor program, defining the program ID and instruction handlers

## Key Components

1. `Config`: Stores AMM state including authority, seed, fee, token mints, and bump seeds
2. `ConstantProduct`: Implements the constant product formula (x * y = k) for pricing and swaps
3. `LiquidityPair`: Represents the token pair in the liquidity pool

## Dependencies

- `anchor-lang`: Solana's Anchor framework
- `anchor-spl`: SPL token integration for Anchor
- `constant-product-curve`: External crate for constant product calculations

## Usage

1. Initialize the AMM with desired parameters (seed, fee, authority)
2. Users can deposit liquidity to earn LP tokens
3. Perform token swaps using the AMM's liquidity pools

## Special Functions

- `ConstantProduct::xy_deposit_amounts_from_l`: Calculates deposit amounts based on the constant product formula
- `ConstantProduct::init`: Initializes the constant product curve for swaps
- `LiquidityPair::swap`: Performs token swaps using the constant product formula

## Installation

1. Ensure you have Rust and Solana CLI installed
2. Clone the repository
3. Run `cargo build` to compile the project

## Testing

(In progress)

## Deployment

(Add steps for deploying the program to Solana devnet or mainnet)

## Security Considerations

- Ensure proper access controls are in place
- Validate all user inputs
- Consider potential edge cases in mathematical calculations


