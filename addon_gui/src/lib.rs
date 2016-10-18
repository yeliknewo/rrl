pub extern crate components as base_comps;
pub extern crate event as event_core;
pub extern crate event_enums as base_events;

pub mod crates {
    pub use base_comps::crates::{cgmath, rustc_serialize, specs, utils};
}

pub use crates::{cgmath, specs, utils};

pub mod components;
pub mod events;
pub mod systems;
