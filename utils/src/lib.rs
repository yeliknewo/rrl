#[macro_use]
extern crate log;
pub extern crate rustc_serialize;
pub extern crate cgmath;
pub extern crate getopts;

pub mod crates {
    pub use ::{rustc_serialize, cgmath, getopts};
}

pub mod fps_counter;
pub mod ortho_helper;
pub mod opts;

pub use fps_counter::FpsCounter;
pub use ortho_helper::OrthographicHelper;
pub use opts::Opter;

pub type Delta = f64;
pub type Coord = f32;
pub type CoordI = i64;
pub type GfxCoord = f32;

use rustc_serialize::{Encodable, Decodable};

pub trait Codable: Encodable + Decodable {

}

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub struct WindowId(pub u32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, RustcEncodable, RustcDecodable)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn map_player(player: Player) -> i32 {
        match player {
            Player::One => 0,
            Player::Two => 1,
        }
    }

    pub fn map_number(number: i32) -> Option<Player> {
        match number {
            0 => Some(Player::One),
            1 => Some(Player::Two),
            _ => None,
        }
    }
}
