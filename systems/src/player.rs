use components::{CompPlayer, CompMoving};
use specs::{System, RunArg, Join};
use utils::{Delta, Coord};
use event::BackChannel;
use event_enums::control_x_player::{ControlToPlayer, ControlFromPlayer};

const SPEED: Coord = 5.0;

pub struct PlayerSystem {
    control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>,
}

impl PlayerSystem {
    pub fn new(control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>)
               -> PlayerSystem {
        PlayerSystem { control_back_channel: control_back_channel }
    }
}

impl System<Delta> for PlayerSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let (players, mut movings) =
            arg.fetch(|w| (w.read::<CompPlayer>(), w.write::<CompMoving>()));

        while let Some(event) = self.control_back_channel.try_recv_to() {
            match event {
                ControlToPlayer::Right(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x = -amount as Coord * SPEED;
                        }
                    }
                }
                ControlToPlayer::Left(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x = amount as Coord * SPEED;
                        }
                    }
                }
                ControlToPlayer::Up(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().y = -amount as Coord * SPEED;
                        }
                    }
                }
                ControlToPlayer::Down(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().y = amount as Coord * SPEED;
                        }
                    }
                }
                ControlToPlayer::Joy(x, y, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x = {
                                x as Coord * SPEED
                            };
                            moving.get_mut_velocity().y = {
                                y as Coord * SPEED
                            };
                        }
                    }
                }
            }
        }
    }
}
