use anchor_lang::prelude::*;
use anchor_spl::token::{InitializeAccount, initialize_account, Transfer, transfer};
use crate::state::Pool;

pub const TOKEN_ACCOUNT_SIZE: usize = 165;
pub const POOL_SEED: &str = "pool";

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

pub fn transfer_from_user_to_pool_vault<'info>(
    signer: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to_vault: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to_vault.to_account_info(),
                authority: signer.to_account_info(),
            },
        ),
        amount,
    )
}

pub fn transfer_from_pool_vault_to_user<'info>(
    pool: &Account<'info, Pool>,
    from_vault: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let bump = [pool.bump];

    let signer_seeds: &[&[u8]] = &[
        POOL_SEED.as_bytes(),
        pool.token0_mint.as_ref(),
        pool.token1_mint.as_ref(),
        &bump,
    ];

    if amount == 0 {
        return Ok(());
    }

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: from_vault.to_account_info(),
                to: to.to_account_info(),
                authority: pool.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )
}