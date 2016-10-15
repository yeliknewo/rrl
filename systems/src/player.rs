use components::{CompMoving, CompPlayer};
use event::BackChannel;
use event_enums::control_x_player::{ControlFromPlayer, ControlToPlayer};
use specs::{Join, RunArg, System};
use utils::{Coord, Delta};

pub struct PlayerSystem {
    control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>,
    speed: Coord,
}

impl PlayerSystem {
    pub fn new(control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>,
               speed: Coord)
               -> PlayerSystem {
        PlayerSystem {
            control_back_channel: control_back_channel,
            speed: speed,
        }
    }
}

impl System<Delta> for PlayerSystem {
    fn run(&mut self,
           arg: RunArg,
           _: Delta) {
        let (players, mut movings) = arg.fetch(|w| (w.read::<CompPlayer>(), w.write::<CompMoving>()));

        while let Some(event) = self.control_back_channel.try_recv_to() {
            match event {
                ControlToPlayer::Right(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x = amount as Coord * self.speed;
                        }
                    }
                }
                ControlToPlayer::Left(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x = -amount as Coord * self.speed;
                        }
                    }
                }
                ControlToPlayer::Up(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().y = amount as Coord * self.speed;
                        }
                    }
                }
                ControlToPlayer::Down(amount, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().y = -amount as Coord * self.speed;
                        }
                    }
                }
                ControlToPlayer::Joy(x, y, player_evt) => {
                    for (player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x = {
                                x as Coord * self.speed
                            };
                            moving.get_mut_velocity().y = {
                                y as Coord * self.speed
                            };
                        }
                    }
                }
            }
        }
    }
}
