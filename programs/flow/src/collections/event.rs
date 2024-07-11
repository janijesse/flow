use anchor_lang::prelude::*;


#[acount]
#[derive(InitSpace)]
pub struct Event {
    
    // Datos básicos
    #[max_len(50)]
    nombre: String,
    precio_ticket: u64,
    activo: bool,

    // Cuentas
    organizador: Pubkey,
    moneda_pago: Pubkey,

    // bumps de la PDAs
    pub bump_evento: u8,
    pub bump_flow_token: u8,
    pub bump_tesoreria: u8,
    pub bump_boveda_ganancias: u8,
}

impl Event {
    
    // Seeds
    pub const SEED_EVENTO: &'static str = "evento";
    pub const SEED_FLOW_TOKEN: &'static str = "floe_token";
    pub const SEED_TESORERIA: &'static str = "tesoreria";
    pub const SEED_BOVEDA_GANANCIAS: &'static str = "boveda_ganancias";
}
