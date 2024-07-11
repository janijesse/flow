use {crate::collections::Event, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct CloseEvent<'info> {
    #[account(
        mut,
        seeds = [
            Event::SEED_EVENTO.as_ref(), 
            evento.organizador.key().as_ref() 
        ],
        bump = evento.event_bump,
    )]
    pub evento: Box<Account<'info, Evento>>,
    
    #[account(mut)]
    pub organizador: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CloseEvent>
) -> Result<()> {
    ctx.accounts.evento.activo = false;
    Ok(())
}