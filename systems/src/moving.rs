use components::{Transform, CompMoving};
use specs::{System, RunArg, Join};
use utils::{Delta, Coord};

const FRICTION: f32 = 1.0;

pub struct MovingSystem {

}

impl MovingSystem {
    pub fn new() -> MovingSystem {
        MovingSystem {}
    }
}

impl System<Delta> for MovingSystem {
    fn run(&mut self, args: RunArg, delta_time: Delta) {
        let (mut transforms, mut movings) =
            args.fetch(|w| (w.write::<Transform>(), w.write::<CompMoving>()));

        for (mut transform, mut moving) in (&mut transforms, &mut movings).iter() {
            transform.add_pos(moving.get_velocity().clone() * delta_time as Coord);

            *moving.get_mut_velocity() *= FRICTION;
        }
    }
}
