pub mod state;
pub mod bonding_curve;
pub mod migration;

// We add a bit of margin to each account data. This is helpful if in the future we add new fields to the existing structs.
// With this additional space we would not need to send `realloc` instructions which are inconvenient.
pub const SPACE_MARGIN: usize = 1000;
