use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mint_test {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let simple_account = &mut ctx.accounts.simple_account;

        simple_account.authority = ctx.accounts.payer.key();

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(bump: u8, pda_auth_bump: u8)]
pub struct Initialize<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        mint::decimals = 9,
        mint::authority = payer,
        mint::freeze_authority = payer,
        seeds=[b"token_mint_pda"],
        bump = bump,
        payer = payer
    )]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = payer)]
    pub simple_account: ProgramAccount<'info, SimpleAccount>,

    #[account(
        init_if_needed,
        associated_token::authority = payer,
        associated_token::mint = mint,
        payer = payer
    )]
    pub assoc_at: Account<'info, TokenAccount>,

    #[account(seeds = [b"pda_authority"], bump = pda_auth_bump)]
    pub mint_authority: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    pub payer: Signer<'info>,

    pub pool_token_mint: Account<'info, Mint>,
}

#[account]
#[derive(Default)]
pub struct SimpleAccount {
    pub authority: Pubkey,
}
