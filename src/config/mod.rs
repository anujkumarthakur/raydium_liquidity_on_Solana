use dotenv::dotenv;
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub wallet_path: String,
    pub liquidity_program_id: String,
    pub pool_lp_token_mint: String,
    pub user_coin_token_account: String,
    pub user_pc_token_account: String,
    pub pool_authority: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok(); // Load environment variables from `.env` file

        Self {
            rpc_url: env::var("RPC_URL").expect("RPC_URL not found in .env"),
            wallet_path: env::var("WALLET_PATH").expect("WALLET_PATH not found in .env"),
            liquidity_program_id: env::var("LIQUIDITY_PROGRAM_ID").expect("Program ID not found"),
            pool_lp_token_mint: env::var("POOL_LP_TOKEN_MINT").expect("LP token mint not found"),
            user_coin_token_account: env::var("USER_COIN_TOKEN_ACCOUNT").expect("Coin account not found"),
            user_pc_token_account: env::var("USER_PC_TOKEN_ACCOUNT").expect("PC account not found"),
            pool_authority: env::var("POOL_AUTHORITY").expect("Pool authority not found"),
        }
    }
}
