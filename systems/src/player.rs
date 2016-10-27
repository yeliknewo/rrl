use event_core::duo_channel::DuoChannel;
// use event_enums::control_x_player::{ControlFromPlayer, ControlToPlayer};
use specs::{RunArg, System};
use std::any::Any;
use utils::{Coord, Delta};

#[allow(dead_code)]
pub struct PlayerSystem<ID, F1>
    where ID: Send + Eq + Ord,
          F1: Send + Fn(&mut PlayerSystem<ID, F1>, RunArg)
{
    // control_back_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
    control_channel_index: usize,
    channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>,
    event_handler: Option<F1>,
    speed: Coord,
}

impl<ID, F1> PlayerSystem<ID, F1>
    where ID: Send + Eq + Ord,
          F1: Send + Fn(&mut PlayerSystem<ID, F1>, RunArg)
{
    pub fn new(channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>, control_channel_id: ID, speed: Coord, event_handler: F1) -> PlayerSystem<ID, F1> {
        PlayerSystem {
            // control_back_channel: control_back_channel,
            control_channel_index: channels.binary_search_by_key(&&control_channel_id, |item| item.get_id()).unwrap_or_else(|err| panic!("{:?}", err)),
            channels: channels,
            speed: speed,
            event_handler: Some(event_handler),
        }
    }

    pub fn get_speed(&self) -> Coord {
        self.speed
    }

    fn get_mut_control_channel(&mut self) -> Option<&mut DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>> {
        let temp = self.control_channel_index;
        self.channels.get_mut(temp)
    }

    pub fn get_mut_speed(&mut self) -> &mut Coord {
        &mut self.speed
    }
}

impl<ID, F1> System<Delta> for PlayerSystem<ID, F1>
    where ID: Send + Eq + Ord,
          F1: Send + Fn(&mut PlayerSystem<ID, F1>, RunArg)
{
    fn run(&mut self, arg: RunArg, _: Delta) {

        let handler = self.event_handler.take().unwrap_or_else(|| panic!("Event Handler was none"));
        handler(self, arg);
        self.event_handler = Some(handler);
    }
}
