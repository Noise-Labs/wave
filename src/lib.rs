#[macro_use]
extern crate nom;

#[macro_use]
mod macros;
pub mod error;
pub mod sample;

pub mod chunk_header;
pub mod fmt_chunk;
pub mod data_chunk;



use chunk_header::*;
