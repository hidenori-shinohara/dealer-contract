#![no_std]

use soroban_sdk::{contractimpl, vec, Env, symbol, Vec, Symbol};

const CARD: Symbol = symbol!("CARD");

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn next(x: u32) -> u32 {
        // Multiplying it by 2 and taking mod 13 continuously
        // leads to a different number in {1, 2, ..., 13}.
        // This is pretty dumb but is good enough for testing.
        return (2 * x) % 13;
    }
    pub fn deal(env: Env) -> Vec<u32> {
        // Get the current count.
        let card: u32 = env
            .storage()
            .get(&CARD)
            .unwrap_or(Ok(1)) // If no value set, assume 1.
            .unwrap(); // Panic if the value of CARD is not u32.

        // Not exactly random, but looks random enough for this purpose.
        let _card1: u32 = Self::next(card);
        let _card2: u32 = Self::next(_card1);
        // Save the count.
        env.storage().set(&CARD, &_card2);
        vec![&env, _card1, _card2]
    }
}
