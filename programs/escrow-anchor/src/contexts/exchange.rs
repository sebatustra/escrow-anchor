use anchor_lang::prelude::*;
use anchor_spl::token::{CloseAccount, Token, TokenAccount, Transfer};

use crate::EscrowAccount;

#[derive(Accounts)]
pub struct Exchange<'info> {
    /// CHECK:
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.taker_amount <= taker_deposit_token_account.amount,
        constraint = escrow_account.initializer_deposit_token_account == *initializer_deposit_token_account.to_account_info().key,
        constraint = escrow_account.initializer_receive_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_account.initializer_key == *initializer.key,
        close = initializer
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    /// CHECK:
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
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
            self.token_program.to_account_info().clone(), 
            cpi_accounts
        )
    }

    pub fn into_transfer_to_initializer_account(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.taker_deposit_token_account.to_account_info().clone(),
            to: self.initializer_receive_token_account.to_account_info().clone(),
            authority: self.taker.to_account_info().clone()
        };

        CpiContext::new(
            self.token_program.to_account_info().clone(),
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
            self.token_program.to_account_info().clone(), 
            cpi_accounts
        )
    }
}