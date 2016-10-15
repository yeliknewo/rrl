pub extern crate utils;
pub extern crate specs;

pub mod crates {
    pub use ::{specs, utils};
    pub use utils::crates::{cgmath, rustc_serialize};
}

pub use crates::cgmath;

pub mod camera;
pub mod gui;
pub mod moving;
pub mod player;
pub mod render_data;
pub mod render_id;
pub mod transform;

pub use self::camera::Camera;
pub use self::gui::Gui;
pub use self::moving::CompMoving;
pub use self::player::CompPlayer;
pub use self::render_data::RenderData;
pub use self::render_id::RenderId;
pub use self::transform::Transform;
