use event::BackChannel;
use event_enums::control_x_player::{ControlFromPlayer, ControlToPlayer};
use specs::{RunArg, System};
use utils::{Coord, Delta};

#[allow(dead_code)]
pub struct PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>, RunArg)
{
    control_back_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
    event_handler: Option<F1>,
    speed: Coord,
}

impl<F1> PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>, RunArg)
{
    pub fn new(control_back_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>, speed: Coord, event_handler: F1) -> PlayerSystem<F1> {
        PlayerSystem {
            control_back_channel: control_back_channel,
            speed: speed,
            event_handler: Some(event_handler),
        }
    }

    pub fn get_speed(&self) -> Coord {
        self.speed
    }

    pub fn get_mut_control_back_channel(&mut self) -> &mut BackChannel<ControlToPlayer<f64>, ControlFromPlayer> {
        &mut self.control_back_channel
    }

    pub fn get_mut_speed(&mut self) -> &mut Coord {
        &mut self.speed
    }
}

impl<F1> System<Delta> for PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>, RunArg)
{
    fn run(&mut self, arg: RunArg, _: Delta) {

        let handler = self.event_handler.take().unwrap_or_else(|| panic!("Event Handler was none"));
        handler(self, arg);
        self.event_handler = Some(handler);
    }
}
