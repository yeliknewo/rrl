pub extern crate components as base_comps;

pub mod crates {
    pub use base_comps::crates::{cgmath, rustc_serialize, specs, utils};
}

pub use crates::{cgmath, specs, utils};

pub mod components;
pub mod events;
pub mod systems;
