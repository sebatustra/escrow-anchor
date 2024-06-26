use anchor_lang::prelude::*;
use anchor_spl::token::{self, spl_token::instruction::AuthorityType};

mod contexts;
use contexts::*;

mod states;
use states::*;

declare_id!("4PhY6r2EB5kTfRnhzhksVjsVBetCruNYZXm54x8vQrds");

#[program]
pub mod escrow_anchor {

    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize(
        ctx: Context<Initialize>,
        initializer_amount: u64,
        taker_amount: u64
    ) -> Result<()> {

        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;

        ctx.accounts
            .escrow_account
            .initializer_deposit_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;

        ctx.accounts
            .escrow_account
            .initializer_receive_token_account = *ctx.accounts
            .initializer_receive_token_account
            .to_account_info()
            .key;

        ctx.accounts
            .escrow_account
            .initializer_amount = initializer_amount;

        ctx.accounts
            .escrow_account
            .taker_amount = taker_amount;

        let (vault_authority, _vault_authority_bump) = 
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);

        token::set_authority(
            ctx.accounts.into_set_authority_context(), 
            AuthorityType::AccountOwner, 
            Some(vault_authority)
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(), 
            ctx.accounts.escrow_account.initializer_amount
        )?;

        Ok(())
    }

    pub fn cancel(
        ctx: Context<Cancel>
    ) -> Result<()> {

        let (_vault_authority, vault_authority_bump) = Pubkey::find_program_address(
            &[ESCROW_PDA_SEED], 
            ctx.program_id
        );

        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&[&authority_seeds[..]]), 
            ctx.accounts.escrow_account.initializer_amount
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]])
        )?;

        Ok(())
    }

    pub fn exchange(
        ctx: Context<Exchange>
    ) -> Result<()> {

        let (_vault_authority, vault_authority_bump) = Pubkey::find_program_address(
            &[ESCROW_PDA_SEED],
            ctx.program_id
        );

        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&[&authority_seeds[..]]), 
            ctx.accounts.escrow_account.initializer_amount
        )?;

        token::transfer(
            ctx.accounts
                .into_transfer_to_initializer_account(), 
            ctx.accounts.escrow_account.taker_amount
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]])
        )?;

        Ok(())
    }
}
