use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer, CloseAccount};

use crate::EscrowAccount;

#[derive(Accounts)]
pub struct Exchange<'info> {
    /// CHECK:
    pub taker: AccountInfo<'info>,
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    /// CHECK:
    pub initializer: AccountInfo<'info>,
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub vault_account: Account<'info, TokenAccount>,
    /// CHECK:
    pub vault_authority: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
}

impl<'info> Exchange<'info> {
    pub fn into_transfer_to_taker_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.taker_receive_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone()
        };

        CpiContext::new(
            self.token_program.clone(), 
            cpi_accounts
        )
    }

    pub fn into_transfer_to_initializer_account(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.taker_deposit_token_account.to_account_info().clone(),
            to: self.initializer_receive_token_account.to_account_info().clone(),
            authority: self.taker.clone()
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
            authority: self.vault_authority.clone(),
        };

        CpiContext::new(
            self.token_program.clone(), 
            cpi_accounts
        )
    }
}