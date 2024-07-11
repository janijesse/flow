use {
    anchor_lang::prelude::*,
    anchor_spl::token::*,
    
    crate::collections::Event,
};

#[derive(Accounts)]
pub struct CreateEvent<'info> {

    #[account(mut)]
    pub organizador: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            Event::SEED_EVENTO.as_ref(),
            organizador.key().as_ref(),
        ],
        bump,
        payer = organizador,
        space = 8 + Event::INIT_SPACE
    )]
    pub evento: Box<Account<'info, Event>>,

    pub moneda_pago: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [
            Event::SEED_FLOW_TOKEN.as_ref(),
            evento.key().as_ref()
        ],
        bump,
        payer = organizador, 
        mint::decimals = 0,
        mint::authority = evento, 
    )]
    pub flow_token: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = organizador,
        seeds = [
            Event::SEED_TESORERIA.as_ref(),
            evento.key().as_ref()
        ],
        bump,
        token::mint = flow_token,
        token::authority = evento,
    )]
    pub tesoreria: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = organizador,
        seeds = [
            Event::SEED_BOVEDA_GANANCIAS.as_ref(),
            evento.key().as_ref()
        ],
        bump,
        token::mint = flow_token,
        token::authority = evento,
    )]
    pub boveda_ganancias: Box<Account<'info, TokenAccount>>,
}

pub fn handler(
    ctx: Context<CreateEvent>,
    nombre: String,
    precio_ticket: u64,
) -> Result<()> {
    
    // datos
    ctx.accounts.evento.nombre = nombre;
    ctx.accounts.evento.precio_ticket = precio_ticket;
    ctx.accounts.evento.activo = true;
    ctx.accounts.evento.sponsors = 0;
    
    // cuentas
    ctx.accounts.evento.organizador = ctx.accounts.organizador.key();
    ctx.accounts.evento.moneda_pago = ctx.accounts.moneda_pago.key();
    
    // bumps
    ctx.accounts.evento.bump_evento = ctx.bumps.evento;
    ctx.accounts.evento.bump_flow_token = ctx.bumps.flow_token;
    ctx.accounts.evento.bump_tesoreria = ctx.bumps.tesoreria;
    ctx.accounts.evento.bump_boveda_ganancias = ctx.bumps.boveda_ganancias;

    Ok(())
}
