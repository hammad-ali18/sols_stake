use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use chrono::{DateTime, Local};
// new line here

#[error_code]
pub enum ErrorCode {
    #[msg("The provided staking wallet does not belong to the user.")]
    InvalidUserStakingWalletOwner,
    #[msg("The provided staking wallet does not belong to the admin.")]
    InvalidAdminStakingWalletOwner,

    #[msg("Staking Duration should be greater than 0")]
    InvalidStakingDurtaion,

    #[msg("Unable to Stake: StakingDuration Ends")]
    UnableToStake,

    #[msg("Unable to UnStake: User Staking is not Ended")]
    UnableToUnStake,
}

//array of reward calculation

use super::*;

pub fn staking_initialize(
    ctx: Context<StakingInitialize>,
    start_slot: u64,
    end_slot: u64,
    reward_rate: u64,
    min_staking_duration: i64,
    rewards_per_stake_duration  :Vec<i64>
) -> Result<()> {
    // let mut rewards_per_stake_duration :Vec<u64> =reward_vec;
    msg!("Instruction: Pool");

    let dt: DateTime<Local> = Local::now();
    if min_staking_duration < 0 {
        return err!(ErrorCode::InvalidStakingDurtaion).into();
    }
    let pool_info = &mut ctx.accounts.pool_info;
    pool_info.admin = ctx.accounts.admin.key();
    pool_info.start_slot = start_slot;
    pool_info.end_slot = end_slot;
    pool_info.token = ctx.accounts.staking_token.key();
    pool_info.pool_created_at = dt.timestamp();
    pool_info.min_staking_duration = dt.timestamp() + min_staking_duration;
    pool_info.total_staked_amount = 0;
    pool_info.reward_rate = reward_rate;
    pool_info.rewards_per_stake_duration = rewards_per_stake_duration;
    Ok(())
}
/*
Reward will be based on the total staked amount in pool
*/
pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    msg!("Instruction: Stake");
    let user_info = &mut ctx.accounts.user_info;
    let pool_info = &mut ctx.accounts.pool_info;

    let clock = Clock::get()?;
    let dt: DateTime<Local> = Local::now();

    if dt.timestamp() > pool_info.min_staking_duration {
        return err!(ErrorCode::UnableToStake);
    }

    //if the user has already staked, calculate and mint the reward
    if user_info.amount > 0 {
        
       let mut index =0;
       let mut reward=0;
       //reward multiplier
        pool_info.rewards_per_stake_duration.iter().for_each(|&element|{
         if dt.timestamp() >= element {
            index+=2;
            reward = (user_info.amount * pool_info.reward_rate * index) / pool_info.total_staked_amount;
         }
        });
        

        let cpi_accounts = MintTo {
            mint: ctx.accounts.staking_token.to_account_info(),
            to: ctx.accounts.user_staking_wallet.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, reward);
    }
    //stake
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_staking_wallet.to_account_info(),
        to: ctx.accounts.admin_staking_wallet.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Update the user's staking information
    user_info.amount += amount;
    user_info.deposit_slot = clock.slot;
    user_info.reward_debt = 0;

    //update pool

    pool_info.total_staked_amount += amount;

    Ok(())
}

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    msg!("Instruction: Unstake");
    let user_info = &mut ctx.accounts.user_info;
    let pool_info = &mut ctx.accounts.pool_info;

    let mut index =0;
    let mut reward=0;
    let dt: DateTime<Local> = Local::now();

    if dt.timestamp() < pool_info.min_staking_duration {
        return err!(ErrorCode::UnableToUnStake);
    }

    //reward multiplier
     pool_info.rewards_per_stake_duration.iter().for_each(|&element|{
      if dt.timestamp() >= element {
         index+=2;
         reward = (user_info.amount * pool_info.reward_rate * index) / pool_info.total_staked_amount;
      }
     });
    // Mint the reward tokens to the user's staking wallet

    let cpi_accounts = MintTo {
        mint: ctx.accounts.staking_token.to_account_info(),
        to: ctx.accounts.user_staking_wallet.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, reward)?;
    let cpi_accounts = Transfer {
        from: ctx.accounts.admin_staking_wallet.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, user_info.amount)?;
    user_info.amount = 0;
    user_info.deposit_slot = 0;
    user_info.reward_debt = 0;

    Ok(())
}

pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    msg!("Instruction: Claim Reward");
    let user_info = &mut ctx.accounts.user_info;
    let pool_info = &mut ctx.accounts.pool_info;
    let mut index =0;
    let mut reward=0;
    let dt: DateTime<Local> = Local::now();

    if dt.timestamp() < pool_info.min_staking_duration {
        return err!(ErrorCode::UnableToUnStake);
    }

    //reward multiplier
     pool_info.rewards_per_stake_duration.iter().for_each(|&element|{
      if dt.timestamp() >= element {
         index+=2;
         reward = (user_info.amount * pool_info.reward_rate * index) / pool_info.total_staked_amount;
      }
     });
  

    let reward = (user_info.amount * pool_info.reward_rate) / pool_info.total_staked_amount;
    let cpi_accounts = MintTo {
        mint: ctx.accounts.staking_token.to_account_info(),
        to: ctx.accounts.user_staking_wallet.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, reward)?;
    user_info.reward_debt += reward;
    Ok(())
}

#[derive(Accounts)]
pub struct StakingInitialize<'info> {
    ///CHECK:
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer=admin, space=8+PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(init, payer= user, space= 8+ UserInfo::LEN)]
    pub user_info: Account<'info, UserInfo>,

    #[account(init, payer= user, space= 8+ PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,
    // adding constraint
    #[account(mut,
    constraint = user_staking_wallet.owner == user.key() @ ErrorCode::InvalidUserStakingWalletOwner)]
    pub user_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = admin_staking_wallet.owner == admin.key() @ ErrorCode::InvalidAdminStakingWalletOwner)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    ///CHECK:
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    ///CHECK:
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,

    #[account(init, payer= user, space= 8+ PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,

    // adding constraint
    #[account(mut,
    constraint = user_staking_wallet.owner == user.key() @ ErrorCode::InvalidUserStakingWalletOwner)]
    pub user_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = admin_staking_wallet.owner == admin.key() @ ErrorCode::InvalidAdminStakingWalletOwner)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct ClaimReward<'info> {
    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,

    #[account(init, payer= user, space= 8+PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub user_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PoolInfo {
    pub admin: Pubkey,
    pub start_slot: u64,
    pub end_slot: u64,
    pub token: Pubkey,
    pub pool_created_at: i64,
    pub min_staking_duration: i64,
    pub total_staked_amount: u64,
    pub reward_rate: u64,
    pub rewards_per_stake_duration: Vec<i64>
}

#[account]
pub struct UserInfo {
    pub amount: u64,
    pub reward_debt: u64,
    pub deposit_slot: u64,
}
impl UserInfo {
    pub const LEN: usize = 8 + 8 + 8;
}

impl PoolInfo {
    pub const LEN: usize = 32 + 8 + 8 + 32 + 8 + 8 + 8 + 8 + 64;
}
