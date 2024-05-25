use anchor_lang::prelude::*;

pub mod staking;

use staking::*;


declare_id!("4AJZvLUdKCaYAjjMgWMh9hsbprpi82HEPvUs3XCxpFBo");

#[program]
pub mod day6 {
    use super::*;
   
   //initialize instruction
   pub fn initialize(ctx: Context<Initialize>)-> Result<()>{
    let counter = &mut ctx.accounts.counter;
    counter.count=0;
    msg!("Counter Account Created");
    msg!("Current Count: { }", counter.count);
    Ok(())
   }

   //add increment instruction
   pub fn increment(ctx: Context<Increment>)-> Result<()>{
    let counter = &mut ctx.accounts.counter;
    msg!("Previos counter {}",counter.count);
    counter.count =counter.count.checked_add(1).unwrap();
    msg!("Counter incremented. Current count {}", counter.count);
    Ok(())
   }

   pub fn decrement(ctx: Context<Decrement>)->Result<()>{
    let counter = &mut ctx.accounts.counter;
    msg!("Previous counter{}",counter.count);
    counter.count = counter.count.checked_sub(1).unwrap();
    msg!("Counter decremented. Current count {}", counter.count);

    Ok(())
   }

   //staking.rs calls
   pub fn staking_initialize(ctx: Context<StakingInitialize>,start_slot:u64, end_slot:u64)->Result<()>{
    let _staking_initialize = staking::staking_initialize(ctx,start_slot,end_slot);
    Ok(())
   }
 pub fn stake(ctx: Context<Stake>,amount:u64)->Result<()>{
    let _stake = staking::stake(ctx, amount);
    Ok(())
   }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer= user, space =8+8)]
    pub counter: Account<'info,Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info , System>
}

#[account]
pub struct Counter{
    pub count: u64,
}

#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)]
    pub counter: Account<'info,Counter>,
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct Decrement<'info>{
    #[account(mut)]
    pub counter:Account<'info,Counter>,
    pub user: Signer<'info>
}