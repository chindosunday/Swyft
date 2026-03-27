#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct MathLib;

#[contractimpl]
impl MathLib {
    /// Returns the contract name — used for post-deploy verification.
    pub fn name(_env: Env) -> Symbol {
        Symbol::new(&_env, "math_lib")
    }

    /// Computes sqrt(a * b) using integer square root (Q64.64 fixed-point stub).
    pub fn sqrt_price(a: u128, b: u128) -> u128 {
        let product = a.saturating_mul(b);
        integer_sqrt(product)
    }

    /// Computes the liquidity delta for a given amount and price range (stub).
    pub fn liquidity_delta(amount: u128, price_lower: u128, price_upper: u128) -> u128 {
        if price_upper <= price_lower || price_lower == 0 {
            return 0;
        }
        amount
            .saturating_mul(price_lower)
            .saturating_div(price_upper.saturating_sub(price_lower))
    }
}

fn integer_sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}
