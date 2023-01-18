#![no_std]

use soroban_sdk::{contractimpl, Env, Bytes};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn pub_prms() -> Bytes {
        // This returns bytes containing:
        // - Preprocessed circuits
        // - Public parameters
        // - Eval domains.

        // This is a fairly crappy design, but for this Hackathon,
        // I think it's okay to just hardcode everything.
        let a:Bytes = "hardcoded secret bytes";
        return a;
    }

    // Verify the proof
    pub fn verify(env: Env) -> bool {
        // What does this do?
        //  The "dealer" wants to determine that you're not bluffing.
        //  In case of Blackjack, you can't ask for more cards if your sum is already over 21.
        //  However, the player wants to keep the cards hidden.
        //  This `verify` function takes the user's address and
        //  make sure that the user's making a sensible decision
        //  without learning what their cards are.
        //
        // Implementation:
        //  This function will likely use the host's verify_proof.
        //

        let invoker = env.invoker();

        // TODO: I have no idea how to connect this `invoker`
        // with the `player` contract.

        let _proof:Bytes;
        // call verify_proof(_proof) and return it.


        // Placeholder bool
        true
    }
}
