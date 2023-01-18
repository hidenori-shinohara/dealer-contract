#![no_std]

use soroban_sdk::{contractimpl, vec, Env, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn randgen() -> u32 {
        // Returns a random number between 1 and 13.
        // For now, it always returns 3.
        return 3;
    }
    pub fn deal(env: Env) -> Vec<u32> {
        let _card1: u32 = Self::randgen();
        let _card2: u32 = Self::randgen();
        vec![&env, _card1, _card2]
    }
}
