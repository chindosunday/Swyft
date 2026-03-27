#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct Router;

#[contractimpl]
impl Router {
    /// Returns the contract name — used for post-deploy verification.
    pub fn name(_env: Env) -> Symbol {
        Symbol::new(&_env, "router")
    }

    /// Initialises the router with the pool-factory address.
    pub fn initialize(env: Env, factory: Address) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "factory"), &factory);
    }

    /// Returns the registered factory address.
    pub fn get_factory(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "factory"))
            .unwrap()
    }
}
