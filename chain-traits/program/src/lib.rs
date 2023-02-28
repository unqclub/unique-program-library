use solana_program::declare_id;

pub mod entrypoint;
pub mod errors;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

declare_id!("78kjgzhBwUrtCFRfwMC5acohpmrdhHLq44aoG8WQvBUd");
