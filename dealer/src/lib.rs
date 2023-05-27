#![no_std]

use soroban_sdk::{contractimpl, symbol, vec, Bytes, BytesN, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    // Verify the proof of a player
    pub fn verify(env: Env, contract_id: BytesN<32>) -> Symbol {
        let proof: Bytes =
            env.invoke_contract::<Bytes>(&contract_id, &symbol!("proof"), vec![&env]);
        let decision: u32 =
            env.invoke_contract::<u32>(&contract_id, &symbol!("decision"), vec![&env]);

        match env.verify(proof, vec![&env, decision]) {
            true => symbol!("TRUTH"),
            false => symbol!("LIE"),
        }
    }
}
