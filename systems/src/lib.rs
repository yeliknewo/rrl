#[macro_use]
pub extern crate log;

pub extern crate components;
pub extern crate event;
pub extern crate event_enums;
pub extern crate neural;

pub mod crates {
    pub use ::{components, event_enums, event, neural};
    pub use event_enums::crates::{getopts, graphics, gfx, gfx_device_gl, glutin,
                                  gfx_window_glutin, sdl2, gfx_window_sdl, find_folder, image,
                                  utils, cgmath, rustc_serialize};
    pub use neural::crates::rand;
    pub use components::crates::specs;
}

pub use crates::{getopts, rustc_serialize, specs, utils, cgmath, gfx, graphics, find_folder, rand};

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
