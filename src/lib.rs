pub mod cli;
pub mod generator;
pub mod png;
pub mod svg;
pub mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use crate::generator::Generator;
