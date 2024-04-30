use anchor_lang::prelude::*;

#[program]

pub mod day14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //         let the_signer1: &mut Signer = &mut ctx.accounts.signer1;


        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        // Function logic....

        msg!("The signer1: {:?}", *the_signer1.key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
}