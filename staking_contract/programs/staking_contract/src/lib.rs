use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("YourProgramIDHere");

#[program]
pub mod staking_contract {
    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        let user_account = &mut ctx.accounts.user_account;

        require!(amount > 0, CustomError::InvalidAmount);

        // transfer tokens from user to staking account
        token::transfer(
            ctx.accounts
                .transfer_context()
                .with_signer(&[]),
            amount,
        )?;

        staking_account.amount_staked = staking_account
            .amount_staked
            .checked_add(amount)
            .ok_or(CustomError::Overflow)?;

        // unlock tiers
        if staking_account.amount_staked >= 25_000_000_000 {
            user_account.unlocked_tier1 = true;
        }
        if staking_account.amount_staked >= 100_000_000_000 {
            user_account.unlocked_tier2 = true;
        }

        emit!(Staked {
            user: ctx.accounts.user.key(),
            amount,
        });

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        let user_account = &mut ctx.accounts.user_account;

        require!(amount > 0 && staking_account.amount_staked >= amount, CustomError::InvalidAmount);

        token::transfer(
            ctx.accounts
                .transfer_back_context()
                .with_signer(&[]),
            amount,
        )?;

        staking_account.amount_staked = staking_account
            .amount_staked
            .checked_sub(amount)
            .ok_or(CustomError::Overflow)?;

        // adjust tiers
        if staking_account.amount_staked < 25_000_000_000 {
            user_account.unlocked_tier1 = false;
        }
        if staking_account.amount_staked < 100_000_000_000 {
            user_account.unlocked_tier2 = false;
        }

        emit!(Unstaked {
            user: ctx.accounts.user.key(),
            amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 32 + 8)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut, token::mint = cgi_mint, token::authority = user)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut, token::mint = cgi_mint, token::authority = staking_account)]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub cgi_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut, token::mint = cgi_mint, token::authority = vault_token_account)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, token::mint = cgi_mint, token::authority = user)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub cgi_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakingAccount {
    pub amount_staked: u64,
}

#[account]
pub struct UserAccount {
    pub unlocked_tier1: bool,
    pub unlocked_tier2: bool,
}

#[event]
pub struct Staked {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Unstaked {
    pub user: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Math overflow")]
    Overflow,
}

impl<'info> Stake<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.user_token_account.to_account_info(),
                to: self.vault_token_account.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
}

impl<'info> Unstake<'info> {
    fn transfer_back_context(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.vault_token_account.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.vault_token_account.to_account_info(),
            },
        )
    }
}
