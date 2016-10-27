// use base_events::ai_x_control::{AiFromControl, AiToControl};

// use event_enums::control_x_gui::{ControlFromGui, ControlToGui};
// use event_enums::control_x_player::{ControlFromPlayer, ControlToPlayer};
// use event_enums::main_x_control::{MainFromControl, MainToControl};

// use base_events::control_x_player::ControlToPlayer;
// use base_events::main_x_control::{MainFromControl, MainToControl};

use base_events::control::{FromControl, ToControl};
use event_core::duo_channel::DuoChannel;
use specs::{RunArg, System};
use std::any::Any;
use std::collections::HashMap;
use utils::{Delta, Player};

#[derive(Debug)]
pub struct ControlSystem<ID: Send + Eq + Ord> {
    main_channel_index: usize,
    input_channel_indices: Vec<usize>,
    output_channel_indices: Vec<usize>,
    channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>,
    repeat_map: HashMap<RepeatEvent, FromControl<f64>>,
    time: f64,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum RepeatEvent {
    Horizontal(Player),
    Vertical(Player),
    Joy(Player),
    A(Player),
    B(Player),
    X(Player),
    Y(Player),
    L1(Player),
    L2(Player),
    R1(Player),
    R2(Player),
}

impl<ID> ControlSystem<ID>
    where ID: Send + Eq + Ord
{
    pub fn new(channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>, main_channel_id: ID, input_channel_ids: Vec<ID>, output_channel_ids: Vec<ID>) -> ControlSystem<ID> {
        let mut input_vec = vec![];
        for input_id in input_channel_ids.iter() {
            input_vec.push(channels.binary_search_by_key(&input_id, |item| item.get_id()).unwrap_or_else(|err| panic!("{:?}", err)));
        }
        let mut output_vec = vec![];
        for output_id in output_channel_ids.iter() {
            output_vec.push(channels.binary_search_by_key(&output_id, |item| item.get_id()).unwrap_or_else(|err| panic!("{:?}", err)));
        }
        ControlSystem {
            main_channel_index: channels.binary_search_by_key(&&main_channel_id, |item| item.get_id()).unwrap_or_else(|err| panic!("{:?}", err)),
            input_channel_indices: input_vec,
            output_channel_indices: output_vec,
            channels: channels,
            repeat_map: HashMap::new(),
            time: 0.0,
        }
    }

    fn process_event(&mut self, event: &ToControl<f64>) {
        match event {
            &ToControl::JoyX(x, player, repeat) => self.handle_joy(Some(x), None, player, repeat),
            &ToControl::JoyY(y, player, repeat) => self.handle_joy(None, Some(y), player, repeat),
            event => self.send_repeat(FromControl::from(event)),
        }
    }

    fn handle_joy(&mut self, x_opt: Option<f64>, y_opt: Option<f64>, player: Player, repeat: bool) {
        // TODO Implement This Function
        if repeat {
            self.send_repeat(FromControl::Joy(x_opt, y_opt, player));
        } else {
            self.send_once(FromControl::Joy(x_opt, y_opt, player));
        }
    }

    fn send_repeat(&mut self, event: FromControl<f64>) {
        match &event {
            &FromControl::Up(_, player) => self.repeat_map.insert(RepeatEvent::Vertical(player), event),
            &FromControl::Down(_, player) => self.repeat_map.insert(RepeatEvent::Vertical(player), event),
            &FromControl::Right(_, player) => self.repeat_map.insert(RepeatEvent::Horizontal(player), event),
            &FromControl::Left(_, player) => self.repeat_map.insert(RepeatEvent::Horizontal(player), event),
            &FromControl::Joy(_, _, player) => self.repeat_map.insert(RepeatEvent::Joy(player), event),
            &FromControl::A(player) => self.repeat_map.insert(RepeatEvent::A(player), event),
            &FromControl::B(player) => self.repeat_map.insert(RepeatEvent::B(player), event),
            &FromControl::X(player) => self.repeat_map.insert(RepeatEvent::X(player), event),
            &FromControl::Y(player) => self.repeat_map.insert(RepeatEvent::Y(player), event),
            &FromControl::L1(player) => self.repeat_map.insert(RepeatEvent::L1(player), event),
            &FromControl::L2(player) => self.repeat_map.insert(RepeatEvent::L2(player), event),
            &FromControl::R1(player) => self.repeat_map.insert(RepeatEvent::R1(player), event),
            &FromControl::R2(player) => self.repeat_map.insert(RepeatEvent::R2(player), event),
        };
    }

    fn send_once(&mut self, event: FromControl<f64>) {
        for index in self.output_channel_indices {
            self.channels.get_mut(index).unwrap_or_else(|| panic!("Index was none")).send(Box::new(event));
        }
    }

    fn trigger_repeats(&mut self) {
        for value in self.repeat_map.clone().values() {
            for index in self.output_channel_indices {
                self.channels.get_mut(index).unwrap_or_else(|| panic!("Index was none")).send(Box::new(value.clone()));
            }
        }
    }

    fn get_mut_main_channel(&mut self) -> Option<&mut DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>> {
        let temp = self.main_channel_index;
        self.channels.get_mut(temp)
    }
}

impl<ID> System<Delta> for ControlSystem<ID>
    where ID: Send + Eq + Ord
{
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        self.time += delta_time;

        if self.time >= 300.0 {
            self.time = 0.0;
            self.get_mut_main_channel().unwrap_or_else(|| panic!("Main channel was none")).send(Box::new(FromControl::Save));
        }

        let mut needs_fetch = vec![];

        for i in 0..self.input_channel_indices.len() {
            needs_fetch.push(true);
        }

        while needs_fetch.iter().any(|item| *item) {
            for input_index in self.input_channel_indices {
                if let Some(event) = self.channels.get_mut(input_index).unwrap_or_else(|| panic!("Index was none")).try_recv() {
                    self.process_event(event.downcast_ref().unwrap_or_else(|| panic!("Unable to downcast event")));
                }
            }
        }

        self.trigger_repeats();

        arg.fetch(|_| ());
    }
}
