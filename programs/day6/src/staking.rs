use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use anchor_spl::token::{self,MintTo,Transfer};
use anchor_spl::token_interface::{Mint,TokenAccount,TokenInterface};



    use super::*;

    pub fn staking_initialize(ctx:Context<StakingInitialize>,start_slot: u64,end_slot:u64)-> Result<()>{
        msg!("Instruction: Pool");
        let pool_info = &mut ctx.accounts.pool_info;
        pool_info.admin = ctx.accounts.admin.key();
        pool_info.start_slot = start_slot;
        pool_info.end_slot = end_slot;
        pool_info.token = ctx.accounts.staking_token.key();
        Ok(())

    }

    pub fn stake(ctx:Context<Stake>, amount:u64)->Result<()>{
        msg!("Instruction: Stake");
        let user_info = &mut ctx.accounts.user_info;
        let clock = Clock::get()?;
        
        if user_info.amount >0{
        let reward=(clock.slot -  user_info.deposit_slot) - user_info.reward_debt;

        let cpi_accounts = MintTo{
            mint:ctx.accounts.staking_token.to_account_info(),
            to: ctx.accounts.user_staking_wallet.to_account_info(),
            authority:ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, reward);
        }
        let cpi_accounts = Transfer{
            from : ctx.accounts.user_staking_wallet.to_account_info(),
            to: ctx.accounts.admin_staking_wallet.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program  = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext:: new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        user_info.amount +=amount;
        user_info.deposit_slot = clock.slot;
        user_info.reward_debt =0;


     Ok(())
    }
    

#[derive(Accounts)]
pub struct StakingInitialize<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer=admin, space=8+PoolInfo::LEN)]
    pub pool_info:Account<'info, PoolInfo>,
    #[account(mut)]
    pub staking_token:InterfaceAccount<'info,Mint>,
    #[account(mut)]
    pub admin_staking_wallet:InterfaceAccount<'info,TokenAccount>,
    pub system_program:Program<'info,System>,

}
#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub admin:AccountInfo<'info>,
    #[account(init, payer= user, space= 8+ UserInfo::LEN)]
    pub user_info: Account<'info,UserInfo>,
    #[account(mut)]
    pub user_staking_wallet:InterfaceAccount<'info,TokenAccount>,
    #[account(mut)]
    pub admin_staking_wallet:InterfaceAccount<'info,TokenAccount>,
    #[account(mut)]
    pub staking_token:InterfaceAccount<'info,TokenAccount>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program: Program<'info, System>,



}

#[account]
pub struct PoolInfo{
    pub admin:Pubkey,
    pub start_slot:u64,
    pub end_slot:u64,
    pub token:Pubkey
}

#[account]
pub struct UserInfo{
    pub amount: u64,
    pub reward_debt: u64,
    pub deposit_slot: u64,
}
impl UserInfo{
    pub const LEN:usize =8+8+8;
}

impl PoolInfo{
pub const LEN:usize=32+8+8+32;
}