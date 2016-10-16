use components::{CompMoving, CompPlayer};
use event_enums::control_x_player::ControlToPlayer;
use specs::{Join, RunArg};
use systems::PlayerSystem;
use utils::Coord;

pub fn basic_all_dir<F1>(me: &mut PlayerSystem<F1>, run_arg: RunArg)
    where F1: Send + Fn(&mut PlayerSystem<F1>, RunArg)
{
    let (players, mut movings) = run_arg.fetch(|w| (w.read::<CompPlayer>(), w.write::<CompMoving>()));

    while let Some(event) = me.get_mut_control_back_channel().try_recv_to() {
        match event {
            ControlToPlayer::Right(amount, player_evt) => {
                for (player, mut moving) in (&players, &mut movings).iter() {
                    if player.get_player() == player_evt {
                        moving.get_mut_velocity().x = amount as Coord * me.get_speed();
                    }
                }
            }
            ControlToPlayer::Left(amount, player_evt) => {
                for (player, mut moving) in (&players, &mut movings).iter() {
                    if player.get_player() == player_evt {
                        moving.get_mut_velocity().x = -amount as Coord * me.get_speed();
                    }
                }
            }
            ControlToPlayer::Up(amount, player_evt) => {
                for (player, mut moving) in (&players, &mut movings).iter() {
                    if player.get_player() == player_evt {
                        moving.get_mut_velocity().y = amount as Coord * me.get_speed();
                    }
                }
            }
            ControlToPlayer::Down(amount, player_evt) => {
                for (player, mut moving) in (&players, &mut movings).iter() {
                    if player.get_player() == player_evt {
                        moving.get_mut_velocity().y = -amount as Coord * me.get_speed();
                    }
                }
            }
            ControlToPlayer::Joy(x_opt, y_opt, player_evt) => {
                for (player, mut moving) in (&players, &mut movings).iter() {
                    if player.get_player() == player_evt {
                        if let Some(x) = x_opt {
                            moving.get_mut_velocity().x = x as Coord * me.get_speed();
                        }
                        if let Some(y) = y_opt {
                            moving.get_mut_velocity().y = y as Coord * me.get_speed();
                        }
                    }
                }
            }
            ControlToPlayer::A(_player_evt) => {}
            ControlToPlayer::B(_player_evt) => {}
            ControlToPlayer::X(_player_evt) => {}
            ControlToPlayer::Y(_player_evt) => {}
            ControlToPlayer::L1(_player_evt) => {}
            ControlToPlayer::L2(_player_evt) => {}
            ControlToPlayer::R1(_player_evt) => {}
            ControlToPlayer::R2(_player_evt) => {}
        }
    }
}
