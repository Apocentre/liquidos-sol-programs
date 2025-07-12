use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub const VERSION: u8 = 20; // 2.0
pub const ONE_TOKEN: Decimal = dec!(1_000_000);
pub const LAMPORT_IN_SOL: Decimal = dec!(1_000_000_000);
