#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec,
};

// Struktur data kodok
#[contracttype]
#[derive(Clone, Debug)]
pub struct Frog {
    id: u64,
    name: String,
    color: String,
    price: u64,
    sold: bool,
}

// Key storage
const FROG_DATA: Symbol = symbol_short!("FROGDATA");

#[contract]
pub struct FrogMarketContract;

#[contractimpl]
impl FrogMarketContract {

    // Ambil semua data kodok
    pub fn get_frogs(env: Env) -> Vec<Frog> {
        env.storage()
            .instance()
            .get(&FROG_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah kodok baru
    pub fn create_frog(
        env: Env,
        name: String,
        color: String,
        price: u64,
    ) -> String {

        // Ambil data lama
        let mut frogs: Vec<Frog> = env.storage()
            .instance()
            .get(&FROG_DATA)
            .unwrap_or(Vec::new(&env));

        // Buat kodok baru
        let frog = Frog {
            id: env.prng().gen::<u64>(),
            name,
            color,
            price,
            sold: false,
        };

        // Simpan
        frogs.push_back(frog);

        env.storage().instance().set(&FROG_DATA, &frogs);

        String::from_str(&env, "Kodok berhasil ditambahkan")
    }

    // Beli kodok berdasarkan id
    pub fn buy_frog(env: Env, id: u64) -> String {

        let mut frogs: Vec<Frog> = env.storage()
            .instance()
            .get(&FROG_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..frogs.len() {

            let mut frog = frogs.get(i).unwrap();

            if frog.id == id {

                if frog.sold {
                    return String::from_str(
                        &env,
                        "Kodok sudah terjual"
                    );
                }

                // Tandai terjual
                frog.sold = true;

                // Update data
                frogs.set(i, frog);

                env.storage()
                    .instance()
                    .set(&FROG_DATA, &frogs);

                return String::from_str(
                    &env,
                    "Berhasil membeli kodok"
                );
            }
        }

        String::from_str(&env, "Kodok tidak ditemukan")
    }

    // Hapus kodok
    pub fn delete_frog(env: Env, id: u64) -> String {

        let mut frogs: Vec<Frog> = env.storage()
            .instance()
            .get(&FROG_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..frogs.len() {

            if frogs.get(i).unwrap().id == id {

                frogs.remove(i);

                env.storage()
                    .instance()
                    .set(&FROG_DATA, &frogs);

                return String::from_str(
                    &env,
                    "Kodok berhasil dihapus"
                );
            }
        }

        String::from_str(&env, "Kodok tidak ditemukan")
    }
}

mod test;
