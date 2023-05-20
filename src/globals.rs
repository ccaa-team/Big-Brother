#[cfg(debug_assertions)]
pub const CURSED_BOARD: u64 = 1109426154160533504;
#[cfg(not(debug_assertions))]
pub const CURSED_BOARD: u64 = 1035288261473615973;
#[cfg(debug_assertions)]
pub const THRESHOLD: u8 = 1;
#[cfg(not(debug_assertions))]
pub const THRESHOLD: u8 = 3;
#[cfg(debug_assertions)]
pub const DB_URL: &str = "sqlite://moyai_dbg.db";
#[cfg(not(debug_assertions))]
pub const DB_URL: &str = "sqlite://moyai.db";
