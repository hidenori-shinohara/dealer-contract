#![no_std]

use soroban_sdk::{contractimpl, vec, Env, symbol, Vec, Symbol, log, Bytes};

const CARD1: Symbol = symbol!("CARD1");
const CARD2: Symbol = symbol!("CARD2");
const CHOICE: Symbol = symbol!("CHOICE");
const HIT: Symbol = symbol!("HIT");
const NONE: Symbol = symbol!("NONE");
const STAND: Symbol = symbol!("STAND");

pub struct Contract;

#[contractimpl]
impl Contract {
    // Draw cards
    pub fn draw(env: Env) -> Vec<u32> {
        let card1: u32 = env
            .storage()
            .get(&CARD1)
            .unwrap_or(Ok(1)) // If no value set, assume 1.
            .unwrap(); // Panic if the value of CARD is not u32.

        let card2: u32 = env
            .storage()
            .get(&CARD2)
            .unwrap_or(Ok(12)) // If no value set, assume 1.
            .unwrap(); // Panic if the value of CARD is not u32.

        // Not exactly random, but looks random enough for this purpose.
        let card1 = (2 * card1) % 13;
        let card2 = (4 * card2) % 13;

        // Save the count.
        env.storage().set(&CARD1, &card1);
        env.storage().set(&CARD2, &card2);
        vec![&env, card1, card2]
    }
    pub fn pub_prms() -> Bytes {
        // prepprocessed circuits, public parameters, eval domains.
        // This is the same as the pub_prms() in dealer.
        // This isn't the cleanest design, but it might be good enough
        // for this hackathon.
        // (we can avoid circular dependencies)
        let a:Bytes = "hardcoded bytes";
        return a;
    }
    pub fn proof(env: Env) -> Bytes {
        // This is where we call the host's generate_proof.
        //
        let _prm = Self::pub_prms();
        let p:Bytes = "";
        let choice: Symbol = env
            .storage()
            .get(&CHOICE)
            .unwrap_or(Ok(NONE)) // If no value set, assume "NONE".
            .unwrap(); // Panic if the value of CHOICE is not Symbol.
        let card1: u32 = env
            .storage()
            .get(&CARD1)
            .unwrap_or(Ok(1)) // If no value set, assume 1.
            .unwrap(); // Panic if the value of CARD is not u32.

        let card2: u32 = env
            .storage()
            .get(&CARD2)
            .unwrap_or(Ok(12)) // If no value set, assume 1.
            .unwrap(); // Panic if the value of CARD is not u32.


        // TODO: We now have:
        // - card1, card2: The cards that the user has
        // - choice: the choice that the user made
        // Using these as the input, we can generate a proof using
        // _prm: public parameters.
        return p;
    }
    pub fn hit(env: Env) {
        let choice: Symbol = env
            .storage()
            .get(&CHOICE)
            .unwrap_or(Ok(NONE)) // If no value set, assume "NONE".
            .unwrap(); // Panic if the value of CHOICE is not Symbol.
        if choice == NONE
        {
            env.storage().set(&CHOICE, &HIT);
            let card1: u32 = env
                .storage()
                .get(&CARD1)
                .unwrap_or(Ok(1)) // If no value set, assume 1.
                .unwrap(); // Panic if the value of CARD is not u32.

            let card2: u32 = env
                .storage()
                .get(&CARD2)
                .unwrap_or(Ok(12)) // If no value set, assume 1.
                .unwrap(); // Panic if the value of CARD is not u32.
            if card1 + card2 > 21
            {
                log!(&env, "Your sum {} is > 21, but you decided to hit! Hope no one finds out!", card1 + card2);
            }
            else
            {
                log!(&env, "Your sum {} is <= 21, and you decided to hit! Good luck!", card1 + card2);
            }
        }
        else
        {
            log!(&env, "you already chose to {}, you can't change your mind anymore", choice);
        }
    }
    pub fn stand(env: Env) {
        let choice: Symbol = env
            .storage()
            .get(&CHOICE)
            .unwrap_or(Ok(NONE)) // If no value set, assume "NONE".
            .unwrap(); // Panic if the value of CHOICE is not Symbol.
        if choice == NONE
        {
            env.storage().set(&CHOICE, &STAND);
            log!(&env, "You chose to stand (not take another card). Good luck!");
        }
        else
        {
            log!(&env, "you already chose to {}, you can't change your mind anymore", choice);
        }
    }
}
