#[macro_use]
extern crate log;
pub extern crate neural;
pub extern crate components as base_comps;
pub extern crate event as event_core;
pub extern crate event_enums as base_events;
pub extern crate find_folder;

pub mod crates {
    pub use base_comps::crates::{cgmath, rustc_serialize, specs, utils};
    pub use neural::crates::{num, rand};
}

pub use crates::{cgmath, num, rand, rustc_serialize, specs, utils};

pub mod components;
pub mod events;
pub mod systems;
