mod add_liquidity;
mod remove_liquidity;
mod config;

use solana_sdk::signature::Keypair;
use std::fs::File;
use std::io::Read;
use crate::config::Config;

fn load_keypair(wallet_path: &str) -> Keypair {
    let mut file = File::open(wallet_path).expect("Cannot open wallet file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read wallet file");
    Keypair::from_bytes(&data).expect("Invalid wallet file format")
}

fn main() {
    let config = Config::load();
    let payer = load_keypair(&config.wallet_path);

    add_liquidity::add_liquidity(&config, &payer, 1_000_000, 1_000_000);
    remove_liquidity::remove_liquidity(&config, &payer, 500_000);
}
