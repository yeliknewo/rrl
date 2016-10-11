#[macro_use]
pub extern crate log;
pub extern crate rand;
pub extern crate utils;

pub use crates::rustc_serialize;

pub mod crates {
    pub use utils::crates::{cgmath, rustc_serialize};
    pub use ::{rand, utils};
}

pub mod evolution;
pub mod network;
