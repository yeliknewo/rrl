pub extern crate graphics;

pub mod crates {
    pub use ::graphics;
    pub use graphics::crates::{gfx, gfx_device_gl, find_folder, image, utils, cgmath,
                               rustc_serialize};
    #[cfg(feature = "g_glutin")]
    pub use graphics::crates::{glutin, gfx_window_glutin};
    #[cfg(feature = "g_sdl2")]
    pub use graphics::crates::{sdl2, gfx_window_sdl};
}

pub use crates::gfx;

use gfx::state::Rasterizer;
use graphics::{Packet, Vertex};

pub mod game_3;

pub fn make_square_render() -> Packet {
    let vertices = vec!(
        Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

pub type RenderType = u8;
pub type Layer = u8;
pub type Name = &'static str;
pub type Size = &'static [f32; 2];
pub type Tint = &'static [f32; 4];
pub type Sprite = &'static [f32; 4];
