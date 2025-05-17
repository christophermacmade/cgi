use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("YourProgramID");

#[program]
pub mod staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> ProgramResult {
        let staking_account = &mut ctx.accounts.staking_account;
        let user_account = &mut ctx.accounts.user_account;
        let cgi_token_account = &mut ctx.accounts.cgi_token_account;
        
        // Ensure the amount is valid
        if amount <= 0 {
            return Err(ProgramError::InvalidArgument.into());
        }

        // Transfer $CGI tokens from user to staking account
        token::transfer(ctx.accounts.into(), amount)?;

        // Update the user's staking amount
        staking_account.amount_staked += amount;

        // Determine the tier based on the staked amount
        if staking_account.amount_staked >= 25_000_000_000 {
            // User unlocks Tier 1
            user_account.unlocked_tiers.push(1);
        }
        if staking_account.amount_staked >= 100_000_000_000 {
            // User unlocks Tier 2
            user_account.unlocked_tiers.push(2);
        }

        emit!(Staked {
            user: ctx.accounts.user.key(),
            amount,
            tier: user_account.unlocked_tiers.clone(),
        });

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
        let staking_account = &mut ctx.accounts.staking_account;
        let user_account = &mut ctx.accounts.user_account;
        let cgi_token_account = &mut ctx.accounts.cgi_token_account;

        // Ensure the amount is valid
        if amount <= 0 || staking_account.amount_staked < amount {
            return Err(ProgramError::InvalidArgument.into());
        }

        // Transfer $CGI tokens from staking account to user
        token::transfer(ctx.accounts.into(), amount)?;

        // Update the user's staking amount
        staking_account.amount_staked -= amount;

        // Adjust the unlocked tiers if necessary
        if staking_account.amount_staked < 25_000_000_000 {
            user_account.unlocked_tiers.retain(|&tier| tier != 1);
        }
        if staking_account.amount_staked < 100_000_000_000 {
            user_account.unlocked_tiers.retain(|&tier| tier != 2);
        }

        emit!(Unstaked {
            user: ctx.accounts.user.key(),
            amount,
            remaining_stake: staking_account.amount_staked,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut, token::mint = cgi_mint, token::authority = user_account)]
    pub cgi_token_account: Account<'info, TokenAccount>,
    pub cgi_mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut, token::mint = cgi_mint, token::authority = user_account)]
    pub cgi_token_account: Account<'info, TokenAccount>,
    pub cgi_mint: Account<'info, Mint>,
}

#[account]
pub struct StakingAccount {
    pub amount_staked: u64,
}

#[account]
pub struct UserAccount {
    pub unlocked_tiers: Vec<u8>, // List of unlocked tiers
}

#[event]
pub struct Staked {
    pub user: Pubkey,
    pub amount: u64,
    pub tier: Vec<u8>,
}

#[event]
pub struct Unstaked {
    pub user: Pubkey,
    pub amount: u64,
    pub remaining_stake: u64,
}
