#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Decimal,
    Symbol,
    Name,
    Balance(Address),
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    #[allow(clippy::too_many_arguments)]
    pub fn initialize(env: &Env, admin: Address, decimal: u32, symbol: String, name: String) {
        // Verify contract is not already initialized
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        // Store contract data
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Decimal, &decimal);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Name, &name);
    }

    pub fn mint(env: &Env, admin: Address, to: Address, amount: i128) {
        // Verify admin
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("not authorized");
        }
        admin.require_auth();

        // Update balance
        let balance = Self::balance(env, to.clone());
        env.storage()
            .instance()
            .set(&DataKey::Balance(to.clone()), &(balance + amount));
    }

    pub fn balance(env: &Env, id: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(id.clone()))
            .unwrap_or(0)
    }
}

mod test;
