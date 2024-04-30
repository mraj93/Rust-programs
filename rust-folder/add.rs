use anchor_lang::prelude::*;

declare_id!("GYckuj9eqg5QetocF4KteRTTFJ6pYc2dr3EaTKbsLC6R");

#[program]
pub mod day_3 {
    use super::*;

    pub fn add(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        let sum = a+b;
        msg!("The sum is {}", sum);
        Ok(())
    }

    pub fn sub(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        let difference = a-b;
        msg!("The difference is {}", difference);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
