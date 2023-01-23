#![cfg(test)]

use super::*;
use soroban_sdk::Env;

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Player);
    let player = PlayerClient::new(&env, &contract_id);
    let cards: Vec<u32> = player.draw_cards();
    assert_eq!(cards, vec![&env, 1, 7, 12]);

    let decision: Symbol = player.decide(&0_u32);
    assert_eq!(decision, STAND);

    let proof = player.proof();
    assert_eq!(proof.len(), 1040);
}
