use {
    anchor_lang::prelude::*,

    crate::collections::Event,
};


#[derive(Accounts)]
pub struct CloseEvent<'info> {

    #[account(mut)]
    pub organizador: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            Event::SEED_EVENTO.as_ref(), 
            organizador.key().as_ref() 
        ],
        bump = evento.bump_evento,
    )]
    pub evento: Box<Account<'info, Event>>,    
}

pub fn handler(ctx: Context<CloseEvent>) -> Result<()> {
    
    ctx.accounts.evento.activo = false;
    
    Ok(())
}
