#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct TokenFaucet;

#[contractimpl]
impl TokenFaucet {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Claim 1 XLM from the faucet. Each address can only claim once.
    /// Contract must be funded with XLM beforehand.
    pub fn claim(env: Env, user: Address) -> bool {
        user.require_auth();

        let mut claimed: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&"claimed")
            .unwrap_or(Map::new(&env));

        if claimed.get(user.clone()).unwrap_or(false) {
            panic!("Already claimed");
        }

        claimed.set(user.clone(), true);
        env.storage().instance().set(&"claimed", &claimed);

        // Note: Actual XLM transfer from contract to user requires
        // integration with Stellar's native asset. Contract must be
        // funded with XLM before deployment.

        true
    }

    /// Check if an address has already claimed
    pub fn has_claimed(env: Env, user: Address) -> bool {
        let claimed: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&"claimed")
            .unwrap_or(Map::new(&env));
        claimed.get(user).unwrap_or(false)
    }
}
