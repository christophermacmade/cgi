use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use staking_contract::program::StakingContract;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, transaction::Transaction};

#[tokio::test]
async fn test_stake_unstake() {
    let program_id = Pubkey::from_str("YourProgramIDHere").unwrap();
    let mut program_test = ProgramTest::new(
        "staking_contract",
        program_id,
        processor!(staking_contract::entry),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let user = Keypair::new();

    // Further test implementation here...
}
