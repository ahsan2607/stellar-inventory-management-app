#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk inventory barang
#[contracttype]
#[derive(Clone, Debug)]
pub struct Item {
    id: u64,
    name: String,
    quantity: u32,
    location: String,
}

// Storage key untuk inventory
const INVENTORY_DATA: Symbol = symbol_short!("INVENTORY");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {

    // Ambil semua item
    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah item baru ke inventory
    pub fn add_item(env: Env, name: String, quantity: u32, location: String) -> String {
        let mut items: Vec<Item> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        let item = Item {
            id: env.prng().gen::<u64>(),
            name,
            quantity,
            location,
        };

        items.push_back(item);

        env.storage().instance().set(&INVENTORY_DATA, &items);

        String::from_str(&env, "Item berhasil ditambahkan")
    }

    // Hapus item berdasarkan id
    pub fn remove_item(env: Env, id: u64) -> String {
        let mut items: Vec<Item> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);

                env.storage().instance().set(&INVENTORY_DATA, &items);
                return String::from_str(&env, "Item berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }

    // Update jumlah stok
    pub fn update_quantity(env: Env, id: u64, new_quantity: u32) -> String {
        let mut items: Vec<Item> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {
                item.quantity = new_quantity;

                items.set(i, item);
                env.storage().instance().set(&INVENTORY_DATA, &items);

                return String::from_str(&env, "Stok berhasil diperbarui");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
}

mod test;