pub extern crate graphics;

pub mod crates {
    pub use ::graphics;
    pub use graphics::crates::{getopts, gfx, gfx_device_gl, glutin, gfx_window_glutin, sdl2,
                               gfx_window_sdl, find_folder, image, utils, cgmath, rustc_serialize};
}

pub use crates::utils;

pub mod ai_x_control;
pub mod control_x_player;
pub mod feeder_x_ai;
pub mod main_x_ai;
pub mod main_x_control;
pub mod main_x_game;
pub mod main_x_render;
pub mod score_x_feeder;
