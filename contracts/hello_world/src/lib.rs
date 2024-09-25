#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol, String, symbol_short};

// Define Symbols for storage keys
const GAME_LIST: Symbol = symbol_short!("GAME_LIST");
const ASSET_LIST: Symbol = symbol_short!("ASSET_LI");
const LUMENS_BALANCE: Symbol = symbol_short!("LUM_BAL");

#[contract]
pub struct GamingPlatformContract;

#[contractimpl]
impl GamingPlatformContract {
    /// This function allows developers to publish a new game on the platform.
    /// It records the game information and makes it available to players.
    pub fn publish_game(env: Env, game_id: u64, game_title: String, game_description: String) {
        let game_info = (game_title.clone(), game_description.clone());
        env.storage().instance().set(&game_id, &game_info);

        log!(&env, "Game Published: ID = {}, Title = {}", game_id, game_title);
    }

    /// This function allows players to purchase in-game assets using Lumens.
    /// It validates the player's balance and updates asset ownership.
    pub fn purchase_asset(env: Env, asset_id: u64, player_id: u64, asset_price: u64) {
        let player_balance: u64 = env.storage().instance().get(&LUMENS_BALANCE).unwrap_or(0);

        if player_balance < asset_price {
            log!(&env, "Insufficient funds for player ID: {}", player_id);
            panic!("Insufficient funds");
        }

        // Deduct the asset price from player's balance
        env.storage().instance().set(&LUMENS_BALANCE, &(player_balance - asset_price));

        // Record the asset purchase
        env.storage().instance().set(&asset_id, &player_id);

        log!(&env, "Asset Purchased: Asset ID = {}, Player ID = {}", asset_id, player_id);
    }
}
