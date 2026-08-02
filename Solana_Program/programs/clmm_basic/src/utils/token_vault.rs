use anchor_lang::prelude::*;
use anchor_spl::token::{InitializeAccount, initialize_account};

pub const TOKEN_ACCOUNT_SIZE: usize = 165;

pub fn create_token_vault_account<'info>(
    vault_account: &mut AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    owner: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    rent: &Sysvar<'info, Rent>,
) -> Result<()> {
    let rent_exempt_lamports = rent.minimum_balance(TOKEN_ACCOUNT_SIZE);

    let create_account_ix = anchor_lang::system_program::CreateAccount {
        from: payer.to_account_info(),
        to: vault_account.to_account_info(),
    };
    let create_account_ctx = CpiContext::new(system_program.to_account_info(), create_account_ix);
    anchor_lang::system_program::create_account(
        create_account_ctx,
        rent_exempt_lamports,
        TOKEN_ACCOUNT_SIZE as u64,
        &token_program.key(),
    )?;

    let init_account_ix = InitializeAccount {
        account: vault_account.to_account_info(),
        mint: mint.to_account_info(),
        authority: owner.to_account_info(),
        rent: rent.to_account_info(),
    };
    let init_account_ctx = CpiContext::new(token_program.to_account_info(), init_account_ix);
    initialize_account(init_account_ctx)?;

    Ok(())
}