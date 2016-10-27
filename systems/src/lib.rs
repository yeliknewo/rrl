#[macro_use]
extern crate log;

extern crate base_comps;
extern crate event_core;
extern crate base_events;
extern crate neural;
extern crate specs;
extern crate utils;
extern crate gfx;
extern crate graphics;

// pub mod crates {
//     pub use ::{components, event, event_enums, neural};
//     pub use components::crates::specs;
//     pub use event_enums::crates::{cgmath, find_folder, getopts, gfx, gfx_device_gl, graphics, image, rustc_serialize, utils};
//     #[cfg(feature = "g_glutin")]
//     pub use event_enums::crates::{gfx_window_glutin, glutin};
//     #[cfg(feature = "g_sdl2")]
//     pub use event_enums::crates::{gfx_window_sdl, sdl2};
//     pub use neural::crates::{num, rand};
// }

// pub use crates::{cgmath, find_folder, getopts, gfx, graphics, num, rand, rustc_serialize, specs, utils};

pub mod control;
pub mod moving;
pub mod player;
pub mod render;

pub use self::control::ControlSystem;
pub use self::moving::MovingSystem;
pub use self::player::PlayerSystem;
pub use self::render::RenderSystem;
