#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct PoolKey {
    pub token_a: Address,
    pub token_b: Address,
    pub fee_tier: u32,
}

#[contract]
pub struct PoolFactory;

#[contractimpl]
impl PoolFactory {
    /// Returns the contract name — used for post-deploy verification.
    pub fn name(_env: Env) -> Symbol {
        Symbol::new(&_env, "pool_factory")
    }

    /// Initialises the factory with the math-lib and owner addresses.
    pub fn initialize(env: Env, owner: Address, math_lib: Address) {
        owner.require_auth();
        env.storage().instance().set(&Symbol::new(&env, "owner"), &owner);
        env.storage().instance().set(&Symbol::new(&env, "math_lib"), &math_lib);
    }

    /// Returns all deployed pool addresses keyed by (token_a, token_b, fee_tier).
    pub fn get_pools(env: Env) -> Map<Symbol, Address> {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "pools"))
            .unwrap_or(Map::new(&env))
    }
}
