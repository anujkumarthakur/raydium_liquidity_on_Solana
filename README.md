# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.16.2/install)"

# Add Solana to your path
export PATH="/home/$USER/.local/share/solana/install/active_release/bin:$PATH"

# Install SPL Token CLI
cargo install spl-token-cli

2. Configure Solana CLI to Use Devnet
solana config set --url https://api.devnet.solana.com

3. Generate Your Wallet (Keypair)
solana-keygen new --outfile ~/keypair.json

4. Airdrop Some Devnet SOL
solana airdrop 2 <Your_Wallet_Address>

5. Create SPL Token Accounts
Create Token Mint (POOL_LP_TOKEN_MINT)
spl-token create-token

# Create a token account for your coin token
spl-token create-account <COIN_TOKEN_MINT_ADDRESS>

# Create a token account for your PC token
spl-token create-account <PC_TOKEN_MINT_ADDRESS>

7. Set LIQUIDITY_PROGRAM_ID
    Standard AMM Program ID: CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW
8. Verify Everything

# List all token accounts associated with your wallet
spl-token accounts

# Check token supply for the created mint
spl-token supply <POOL_LP_TOKEN_MINT>


