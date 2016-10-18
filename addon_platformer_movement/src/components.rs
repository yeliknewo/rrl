use cgmath::Vector3;
use specs::{Component, VecStorage, World};
use utils::Coord;

pub fn register_components(world: &mut World) {
    world.register::<CompPlatMoving>();
}

pub struct CompPlatMoving {
    velocity: Vector3<Coord>,
}

impl CompPlatMoving {
    pub fn new(velocity: Vector3<Coord>) -> CompPlatMoving {
        CompPlatMoving {
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

impl Component for CompPlatMoving {
    type Storage = VecStorage<CompPlatMoving>;
}
