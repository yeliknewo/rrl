#[macro_use]
pub extern crate log;

pub extern crate components;
pub extern crate event;
pub extern crate event_enums;
pub extern crate neural;

pub mod crates {
    pub use ::{components, event, event_enums, neural};
    pub use components::crates::specs;
    pub use event_enums::crates::{cgmath, find_folder, getopts, gfx, gfx_device_gl, graphics, image, rustc_serialize, utils};
    #[cfg(feature = "g_glutin")]
    pub use event_enums::crates::{gfx_window_glutin, glutin};
    #[cfg(feature = "g_sdl2")]
    pub use event_enums::crates::{gfx_window_sdl, sdl2};
    pub use neural::crates::{num, rand};
}

pub use crates::{cgmath, find_folder, getopts, gfx, graphics, num, rand, rustc_serialize, specs, utils};

pub mod ai;
pub mod control;
pub mod feeder;
pub mod moving;
pub mod player;
pub mod render;
pub mod score;

pub use self::ai::AiSystem;
pub use self::control::ControlSystem;
pub use self::feeder::FeederSystem;
pub use self::moving::MovingSystem;
pub use self::player::PlayerSystem;
pub use self::render::RenderSystem;
pub use self::score::ScoreSystem;
