#![cfg(test)]
use super::*;
use soroban_sdk::Env;

#[test]
fn test_minting() {
    let env = Env::default();
    
    // Create addresses
    let admin = Address::from_str(&env,"GCJIE7EIKWRMPZOW3OPGYJHQO6E6F5OZVQQNLFJGB7PAHVBS5KBQEUFM");
    let alex = Address::from_str(&env,"GAUSBA6IHKK63ZPCLNQJQ332OFIETXOFSTK7AI7YTKR3M7EPTW7CLR5H");
    let bart = Address::from_str(&env,"GBFAIH5WKAJQ77NG6BZG7TGVGXHPX4SQLIJ7BENJMCVCZSUZPSISCLU5");

    let symbol = String::from_str(&env, "Token");
    let token = String::from_str(&env, "Token");

    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    // Initialize the contract
    client.initialize(&admin, &18, &symbol, &token);

    // Create authorization to mint
    env.mock_all_auths();

    // Mint tokens
    client.mint(&admin, &alex, &1000);
    client.mint(&admin, &bart, &3000);

    // Verify balance
    let alex_balance = client.balance(&alex);
    assert_eq!(alex_balance, 1000);
    let bart_balance = client.balance(&bart);
    assert_eq!(bart_balance, 3000);
}
