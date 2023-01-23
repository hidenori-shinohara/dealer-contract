#![no_std]

use soroban_sdk::{contracterror, contractimpl, log, symbol, vec, Bytes, Env, Symbol, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidDecision = 0,
    NoCards = 1,
    NoProof = 2,
    NoDecision = 3,
}

const CARDS: Symbol = symbol!("CARDS");
const PROOF: Symbol = symbol!("PROOF");
const DECISION: Symbol = symbol!("DECISION");
const STAND: Symbol = symbol!("STAND");
const HIT: Symbol = symbol!("HIT");

pub struct Player;

fn generate_proof(env: &Env, decision: u32) {
    let cards: Vec<u32>;
    if decision == 0 {
        // STAND, generate a dummy proof with zero cards
        cards = vec![env, 0, 0, 0];
    } else {
        cards = env.storage().get_unchecked(&CARDS).unwrap();
    }
    let proof = env.prove(cards, vec![env, decision]);
    env.storage().set(&PROOF, proof);
}

#[contractimpl]
impl Player {
    // Draw cards
    pub fn draw_cards(env: Env) -> Vec<u32> {
        let cards: Vec<u32> = if env.storage().has(&CARDS) {
            let cards: Vec<u32> = env.storage().get_unchecked(&CARDS).unwrap();
            let c0: u32 = cards.get(0).unwrap().unwrap();
            let c1: u32 = cards.get(1).unwrap().unwrap();
            let c2: u32 = cards.get(2).unwrap().unwrap();
            // fake a random generator
            vec![&env, (2 * c0) % 13, (4 * c1) % 13, (7 * c2) % 13]
        } else {
            vec![&env, 1, 7, 12]
        };
        // store the cards
        env.storage().set(&CARDS, cards.clone());
        // reset the decision
        env.storage().set(&DECISION, 999); // an invalid decision
        cards
    }

    pub fn decide(env: Env, input: u32) -> Result<Symbol, Error> {
        env.storage().set(&DECISION, input);
        log!(&env, "Decision has been made -- {}", input);
        log!(&env, "Generating proof..");
        generate_proof(&env, input);
        log!(&env, "{Proof generated.");

        match input {
            0 => Ok(STAND),
            1 => Ok(HIT),
            _ => Err(Error::InvalidDecision),
        }
    }

    pub fn proof(env: Env) -> Result<Bytes, Error> {
        if env.storage().has(&PROOF) {
            Ok(env.storage().get_unchecked(&PROOF).unwrap())
        } else {
            Err(Error::NoProof)
        }
    }

    pub fn decision(env: Env) -> Result<u32, Error> {
        if env.storage().has(&DECISION) {
            Ok(env.storage().get_unchecked(&DECISION).unwrap())
        } else {
            Err(Error::NoDecision)
        }
    }
}

mod test;
