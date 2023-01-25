# Zero-Knowledge Blackjack
This is a zero-knowledge proof application built on Soroban for the hack-a-soroban-winter-2023.

Link to [demo slides](https://docs.google.com/presentation/d/1FjOUJam6Y62MIu6-uSD3-n6_IuOY7nijpInoSQLv0jM/edit?usp=sharing)

# Game Description
Start with 3 random cards, each with a number between 1 and 14.
The goal is to get their sum as close to 21 points without exceeding it.
### Allowed player actions
- STAND(0): do nothing
- HIT(1): ask for another card, with the risk of going over 21
### The key rule:
If the current sum is already > 21, you shall not draw another card (action must be "STAND")

# How to Run
1. Install Soroban
Install `soroban-tools` following instructions from https://github.com/jayz22/soroban-tools/tree/zk-hackathon

2. Build the contracts
- Run `make build`
- `cd target/wasm32-unknown-unknown/release`
You should see two contracts:
- Player (prover) contract: `player.wasm`
- Dealer (verifier) contract: `dealer.wasm`

3. Deploy the contracts
- `soroban deploy --wasm player.wasm --id aaaa1 --ledger-file ledger.json` this deploys the player contract in the sandbox mode. It generates the contract ID `00000000000000000000000000000000000000000000000000000000000aaaa1` for the player contract, and a ledger file `ledger.json` that persists the ledger state.
- `soroban deploy --wasm dealer.wasm --id dddd0 --ledger-file ledger.json` same as above, with dealer contract ID `00000000000000000000000000000000000000000000000000000000000dddd0`

4. Draw cards
- Invoke the player contract: `soroban invoke --id 00000000000000000000000000000000000000000000000000000000000aaaa1 --fn draw_cards --ledger-file ledger.json`. This pseudo-ramdomly generates three cards. (The pseudo ramdom function is very naive and not intended for production, only for this demo.)

5. Make a decision
- Invoke the player contract: `soroban invoke --id 00000000000000000000000000000000000000000000000000000000000aaaa1 --fn decide --arg 1 --ledger-file ledger.json`. The decision follows `--arg`. `0` means to `STAND` and `1` means to `HIT`. 

6. Verify the decision
- Invoke the dealer contract passing in the player contract's ID: `soroban invoke --id 00000000000000000000000000000000000000000000000000000000000dddd0 --fn verify --arg 00000000000000000000000000000000000000000000000000000000000aaaa1 --ledger-file ledger.json`. This will display either `TRUTH` if the player action was legal or `LIE` if otherwise.

Enjoy!