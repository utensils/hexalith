pub mod cli;
pub mod generator;
pub mod svg;
pub mod png;
pub mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use crate::generator::Generator;