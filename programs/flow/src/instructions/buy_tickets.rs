use {
    anchor_lang::prelude::*,
    anchor_spl::token::*,
    
    crate::collections::Event,
};

#[derive(Accounts)]
pub struct BuyTickets<'info> {
    #[account(mut)]
    pub organizador: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            Event::SEED_EVENTO.as_ref(),
            evento.organizador.key().as_ref(),
        ],
        bump = evento.bump_evento,
        constraint = evento.activo == true @ ErrorCode::EventClosed, 
    )]
    pub evento: Box<Account<'info, Event>>,

    #[account(
        mut,
        constraint = ata_moneda_pago.mint == evento.moneda_pago,
        constraint = ata_moneda_pago.amount > 0
    )]
    pub ata_moneda_pago: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            Event::SEED_BOVEDA_GANANCIAS.as_ref(),
            evento.key().as_ref()
        ],
        bump = evento.bump_boveda_ganancias,
    )]
    pub boveda_ganancias: Box<Account<'info, TokenAccount>>,
}

pub fn handler(
    ctx: Context<BuyTickets>,
    cantidad: u64,
  ) -> Result<()> {

    // calculate amount to charge (quantity * token_price)
    let _cantidad = ctx
        .accounts
        .evento
        .precio_ticket
        .checked_mul(cantidad)
        .unwrap();

        // Cobrar
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.ata_moneda_pago.to_account_info(),
                to: ctx.accounts.boveda_ganancias.to_account_info(),
                authority: ctx.accounts.organizador.to_account_info(),
            },
        ),
        _cantidad,
    )?;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("El Evento del FLOW ha cerrado")]
    EventClosed,
}
