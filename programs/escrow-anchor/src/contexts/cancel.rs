use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer, CloseAccount};

use crate::EscrowAccount;

#[derive(Accounts)]
pub struct Cancel<'info> {
    /// CHECK:
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    /// CHECK:
    pub vault_authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.initializer_key == *initializer.key,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
}

impl<'info> Cancel<'info> {
    pub fn into_transfer_to_initializer_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.initializer_deposit_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone()
        };

        CpiContext::new(
            self.token_program.clone(), 
            cpi_accounts
        )
    }

    pub fn into_close_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone()
        };

        CpiContext::new(
            self.token_program.clone(), 
            cpi_accounts
        )
    }
}