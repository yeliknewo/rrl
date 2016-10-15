use event::{BackChannel, FrontChannel};
use event_enums::ai_x_control::{AiFromControl, AiToControl};
use event_enums::control_x_gui::{ControlFromGui, ControlToGui};
use event_enums::control_x_player::{ControlFromPlayer, ControlToPlayer};
use event_enums::main_x_control::{MainFromControl, MainToControl};
use specs::{RunArg, System};
use std::collections::HashMap;
use utils::{Delta, Player};

#[derive(Debug)]
pub struct ControlSystem {
    main_back_channel: BackChannel<MainToControl<f64>, MainFromControl>,
    ai_back_channel: BackChannel<AiToControl<f64>, AiFromControl>,
    player_front_channel: Option<FrontChannel<ControlToPlayer<f64>, ControlFromPlayer>>,
    gui_front_channel: Option<FrontChannel<ControlToGui<f64>, ControlFromGui>>,
    repeat_map: HashMap<RepeatEvent, ControlToPlayer<f64>>,
    time: f64,
}

#[derive(Debug, Hash, Eq, PartialEq)]
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

impl ControlSystem {
    pub fn new(main_back_channel: BackChannel<MainToControl<f64>, MainFromControl>,
               ai_back_channel: BackChannel<AiToControl<f64>, AiFromControl>,
               player_front_channel: FrontChannel<ControlToPlayer<f64>, ControlFromPlayer>,
               gui_front_channel: FrontChannel<ControlToGui<f64>, ControlFromGui>)
               -> ControlSystem {
        ControlSystem {
            main_back_channel: main_back_channel,
            ai_back_channel: ai_back_channel,
            player_front_channel: Some(player_front_channel),
            gui_front_channel: Some(gui_front_channel),
            repeat_map: HashMap::new(),
            time: 0.0,
        }
    }

    fn process_main_event(&mut self,
                          event: MainToControl<f64>) {
        match event {
            MainToControl::JoyX(x, player) => {
                self.handle_joy(Some(x),
                                None,
                                player)
            }
            MainToControl::JoyY(y, player) => {
                self.handle_joy(None,
                                Some(y),
                                player)
            }
            event => self.send_repeat(ControlToPlayer::from(event)),
        }
    }

    fn handle_joy(&mut self,
                  x_opt: Option<f64>,
                  y_opt: Option<f64>,
                  player: Player) {
        // TODO Implement This Function

        self.send_repeat(ControlToPlayer::Joy(x_opt,
                                              y_opt,
                                              player));
    }

    fn process_ai_event(&mut self,
                        event: AiToControl<f64>) {
        self.send_once(ControlToPlayer::from(event));
    }

    fn send_repeat(&mut self,
                   event: ControlToPlayer<f64>) {
        match &event {
            &ControlToPlayer::Up(_, player) => {
                self.repeat_map.insert(RepeatEvent::Vertical(player),
                                       event)
            }
            &ControlToPlayer::Down(_, player) => {
                self.repeat_map.insert(RepeatEvent::Vertical(player),
                                       event)
            }
            &ControlToPlayer::Right(_, player) => {
                self.repeat_map.insert(RepeatEvent::Horizontal(player),
                                       event)
            }
            &ControlToPlayer::Left(_, player) => {
                self.repeat_map.insert(RepeatEvent::Horizontal(player),
                                       event)
            }
            &ControlToPlayer::Joy(_, _, player) => {
                self.repeat_map.insert(RepeatEvent::Joy(player),
                                       event)
            }
            &ControlToPlayer::A(player) => {
                self.repeat_map.insert(RepeatEvent::A(player),
                                       event)
            }
            &ControlToPlayer::B(player) => {
                self.repeat_map.insert(RepeatEvent::B(player),
                                       event)
            }
            &ControlToPlayer::X(player) => {
                self.repeat_map.insert(RepeatEvent::X(player),
                                       event)
            }
            &ControlToPlayer::Y(player) => {
                self.repeat_map.insert(RepeatEvent::Y(player),
                                       event)
            }
            &ControlToPlayer::L1(player) => {
                self.repeat_map.insert(RepeatEvent::L1(player),
                                       event)
            }
            &ControlToPlayer::L2(player) => {
                self.repeat_map.insert(RepeatEvent::L2(player),
                                       event)
            }
            &ControlToPlayer::R1(player) => {
                self.repeat_map.insert(RepeatEvent::R1(player),
                                       event)
            }
            &ControlToPlayer::R2(player) => {
                self.repeat_map.insert(RepeatEvent::R2(player),
                                       event)
            }
        };
    }

    fn send_once(&mut self,
                 event: ControlToPlayer<f64>) {
        self.player_front_channel
            .as_mut()
            .unwrap_or_else(|| panic!("Player Front Channel was none"))
            .send_to(event);
    }

    fn trigger_repeats(&mut self) {
        let mut player_channel = self.player_front_channel
            .take()
            .unwrap_or_else(|| panic!("Player Front Channel was none"));
        let mut gui_channel = self.gui_front_channel
            .take()
            .unwrap_or_else(|| panic!("Gui Front Channel was none"));
        for value in self.repeat_map.values() {
            player_channel.send_to(value.clone());
            gui_channel.send_to(ControlToGui::from(value.clone()));
        }
        self.player_front_channel = Some(player_channel);
        self.gui_front_channel = Some(gui_channel);
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self,
           arg: RunArg,
           delta_time: Delta) {
        self.time += delta_time;

        if self.time >= 300.0 {
            self.time = 0.0;
            self.main_back_channel.send_from(MainFromControl::Save);
        }

        let mut needs_fetch = (true, true);

        while needs_fetch.0 || needs_fetch.1 {
            if let Some(event) = self.main_back_channel.try_recv_to() {
                self.process_main_event(event);
            } else {
                needs_fetch.0 = false;
            }

            if let Some(event) = self.ai_back_channel.try_recv_to() {
                self.process_ai_event(event);
            } else {
                needs_fetch.1 = false;
            }
        }

        self.trigger_repeats();

        arg.fetch(|_| ());
    }
}
