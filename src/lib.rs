#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod safe;
pub mod r#unsafe;

#[cfg(test)]
mod tests;
