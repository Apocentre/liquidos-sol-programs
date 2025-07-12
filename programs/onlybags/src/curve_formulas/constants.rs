use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub const VERSION: u16 = 200; // 2.0.0
pub const ONE_TOKEN: Decimal = dec!(1_000_000);
pub const LAMPORT_IN_SOL: Decimal = dec!(1_000_000_000);
