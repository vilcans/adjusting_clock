#![doc = include_str!("../README.md")]

mod adjustment;
mod clock;

pub use adjustment::{AdjustmentSettings, ConstantRate};
pub use clock::Clock;

#[cfg(test)]
mod test;
