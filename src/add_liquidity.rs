use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use crate::config::Config;
use spl_token::instruction::approve;

pub fn add_liquidity(config: &Config, payer: &Keypair, max_coin_token_amount: u64, max_pc_token_amount: u64) {
    let rpc_client = RpcClient::new(&config.rpc_url);

    // Parse addresses from config
    let program_id = Pubkey::from_str(&config.liquidity_program_id).unwrap();
    let lp_token_mint = Pubkey::from_str(&config.pool_lp_token_mint).unwrap();
    let user_coin_token_account = Pubkey::from_str(&config.user_coin_token_account).unwrap();
    let user_pc_token_account = Pubkey::from_str(&config.user_pc_token_account).unwrap();
    let pool_authority = Pubkey::from_str(&config.pool_authority).unwrap();

    let accounts = vec![
        AccountMeta::new(user_coin_token_account, false),
        AccountMeta::new(user_pc_token_account, false),
        AccountMeta::new(pool_authority, false),
        AccountMeta::new(lp_token_mint, false),
    ];

    let add_liquidity_instruction = Instruction {
        program_id,
        accounts,
        data: vec![],
    };

    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[add_liquidity_instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction successful with signature: {}", signature);
}
