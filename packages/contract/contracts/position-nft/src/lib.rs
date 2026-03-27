#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct PositionNft;

#[contractimpl]
impl PositionNft {
    /// Returns the contract name — used for post-deploy verification.
    pub fn name(_env: Env) -> Symbol {
        Symbol::new(&_env, "position_nft")
    }

    /// Initialises the NFT contract with the authorised minter (router).
    pub fn initialize(env: Env, minter: Address) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "minter"), &minter);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "next_id"), &0u64);
    }

    /// Returns the total number of positions minted.
    pub fn total_supply(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "next_id"))
            .unwrap_or(0u64)
    }
}
