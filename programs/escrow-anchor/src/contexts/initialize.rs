use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, SetAuthority, Transfer};

use crate::EscrowAccount;

#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct Initialize<'info> {
    /// CHECK:
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token_seed".as_ref()],
        bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount >= initializer_amount
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
}

impl<'info> Initialize<'info> {
    pub fn into_set_authority_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone()
        };

        CpiContext::new(
            self.token_program.clone(), 
            cpi_accounts
        )
    }

    pub fn into_transfer_to_pda_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.initializer_deposit_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.clone()
        };

        CpiContext::new(
            self.token_program.clone(), 
            cpi_accounts)
    }
}