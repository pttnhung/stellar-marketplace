#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Map, Vec, String
};

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {

    // 📦 Create listing
    pub fn create_listing(
        env: Env,
        seller: Address,
        item_id: u32,
        name: String,
        price: i128
    ) {
        seller.require_auth();

        let key_items = symbol_short!("ITEMS");

        let mut items: Map<u32, (Address, String, i128)> =
            env.storage().instance().get(&key_items).unwrap_or(Map::new(&env));

        items.set(item_id, (seller, name, price));

        env.storage().instance().set(&key_items, &items);
    }

    // 📊 Get item
    pub fn get_item(env: Env, item_id: u32) -> (Address, String, i128) {
        let key_items = symbol_short!("ITEMS");

        let items: Map<u32, (Address, String, i128)> =
            env.storage().instance().get(&key_items).unwrap_or(Map::new(&env));

        items.get(item_id).unwrap()
    }

    // 📋 Get all items
    pub fn get_all_items(env: Env) -> Vec<(Address, String, i128)> {
        let key_items = symbol_short!("ITEMS");

        let items: Map<u32, (Address, String, i128)> =
            env.storage().instance().get(&key_items).unwrap_or(Map::new(&env));

        let mut list = Vec::new(&env);

        for (_, item) in items.iter() {
            list.push_back(item);
        }

        list
    }
}

stellar contract invoke \
  --id CC6P3EHBP36XZWYNJE2CKV4TAT3HSTEC4CKM77QXE7YNN5EIQGQBAQY5 \
  --source student \
  --network testnet \
  -- \
  create_listing \
  --seller student \
  --item_id 1 \
  --name "Book" \
  --price 100

stellar contract invoke \
  --id CC6P3EHBP36XZWYNJE2CKV4TAT3HSTEC4CKM77QXE7YNN5EIQGQBAQY5 \
  --source student \
  --network testnet \
  -- \
  get_item \
  --item_id 1