use anchor_lang::prelude::*;

use crate::context_accounts::*;

pub fn process_update_authority(
    ctx: Context<UpdateAuthority>,
    new_authority: Pubkey
) -> Result<()> {
    ctx.accounts.project_info.project_owner = new_authority;
    Ok(())
}
