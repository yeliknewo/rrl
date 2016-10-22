use cgmath::Vector3;
use specs::{Component, VecStorage};
use utils::Coord;

pub struct CompMoving {
    velocity: Vector3<Coord>,
}

impl CompMoving {
    pub fn new(velocity: Vector3<Coord>) -> CompMoving {
        CompMoving {
            velocity: velocity,
        }
    }

    pub fn get_velocity(&self) -> &Vector3<Coord> {
        &self.velocity
    }

    pub fn get_mut_velocity(&mut self) -> &mut Vector3<Coord> {
        &mut self.velocity
    }
}

impl Component for CompMoving {
    type Storage = VecStorage<CompMoving>;
}
