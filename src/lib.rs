#![allow(async_fn_in_trait)]
pub mod errors;
pub mod macros;
pub mod messages;

#[must_use]
pub const fn snowflake_to_timestamp(snowflake: u64) -> u64 {
    ///Discord epoch starts from 2015-01-01 00:00:00 UTC
    const DISCORD_EPOCH: u64 = 1_420_070_400_000;
    ((snowflake >> 22) + DISCORD_EPOCH) / 1000
}
