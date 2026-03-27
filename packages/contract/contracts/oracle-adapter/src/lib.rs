#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct OracleAdapter;

#[contractimpl]
impl OracleAdapter {
    /// Returns the contract name — used for post-deploy verification.
    pub fn name(_env: Env) -> Symbol {
        Symbol::new(&_env, "oracle_adapter")
    }

    /// Initialises the adapter with the upstream oracle address.
    pub fn initialize(env: Env, oracle: Address) {
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "oracle"), &oracle);
    }

    /// Returns the registered oracle address.
    pub fn get_oracle(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "oracle"))
            .unwrap()
    }
}
