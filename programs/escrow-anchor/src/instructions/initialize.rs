use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::contexts::Initialize;

pub fn handler(ctx: Context<Initialize>, offer_id: String, amount_of_release_token: u64, amount_of_receieve_token: u64) -> Result<()> {
    ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
    ctx.accounts
        .escrow_account
        .initializer_release_token_account = *ctx
        .accounts
        .initializer_release_token_account
        .to_account_info()
        .key;
    ctx.accounts
        .escrow_account
        .initializer_receive_token_account = *ctx
        .accounts
        .initializer_receive_token_account
        .to_account_info()
        .key;
    ctx.accounts.escrow_account.amount_of_receive_token = amount_of_receieve_token;
    ctx.accounts.escrow_account.amount_of_release_token = amount_of_release_token;
    ctx.accounts.escrow_account.vault_account_bump = *ctx.bumps.get("vault_account").unwrap();
    ctx.accounts.escrow_account.vault_authority_bump = *ctx.bumps.get("vault_authority").unwrap();
    ctx.accounts.escrow_account.escrow_account_bump = *ctx.bumps.get("escrow_account").unwrap();
    ctx.accounts.escrow_account.offer_id = offer_id;


    token::transfer(
        ctx.accounts.into_transfer_to_pda_context(),
        amount_of_release_token,
    )?;

    Ok(())
}