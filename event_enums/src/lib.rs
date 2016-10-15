pub extern crate graphics;

pub mod crates {
    pub use ::graphics;
    pub use graphics::crates::{cgmath, find_folder, getopts, gfx, gfx_device_gl, image, rustc_serialize, utils};
    #[cfg(feature = "g_glutin")]
    pub use graphics::crates::{gfx_window_glutin, glutin};
    #[cfg(feature = "g_sdl2")]
    pub use graphics::crates::{gfx_window_sdl, sdl2};
}

pub use crates::utils;

pub mod ai_x_control;
pub mod control_x_gui;
pub mod control_x_player;
pub mod feeder_x_ai;
pub mod main_x_ai;
pub mod main_x_control;
pub mod main_x_game;
pub mod main_x_render;
pub mod score_x_feeder;
