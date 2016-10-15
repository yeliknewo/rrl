use event::BackChannel;
use event_enums::control_x_player::{ControlFromPlayer, ControlToPlayer};
use specs::{RunArg, System};
use utils::{Coord, Delta};

#[allow(dead_code)]
pub struct PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>,
                 RunArg)
{
    control_back_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
    event_handler: Option<F1>,
    speed: Coord,
}

impl<F1> PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>,
                 RunArg)
{
    pub fn new(control_back_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
               speed: Coord,
               event_handler: F1)
               -> PlayerSystem<F1> {
        PlayerSystem {
            control_back_channel: control_back_channel,
            speed: speed,
            event_handler: Some(event_handler),
        }
    }
}

impl<F1> System<Delta> for PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>,
                 RunArg)
{
    fn run(&mut self,
           arg: RunArg,
           _: Delta) {

        let handler = self.event_handler.take().unwrap_or_else(|| panic!("Event Handler was none"));
        handler(self,
                arg);
        self.event_handler = Some(handler);
        // match event {
        //     ControlToPlayer::Right(amount, player_evt) => {
        //         for (player, mut moving) in (&players, &mut movings).iter() {
        //             if player.get_player() == player_evt {
        //                 moving.get_mut_velocity().x = amount as Coord * self.speed;
        //             }
        //         }
        //     }
        //     ControlToPlayer::Left(amount, player_evt) => {
        //         for (player, mut moving) in (&players, &mut movings).iter() {
        //             if player.get_player() == player_evt {
        //                 moving.get_mut_velocity().x = -amount as Coord * self.speed;
        //             }
        //         }
        //     }
        //     ControlToPlayer::Up(amount, player_evt) => {
        //         for (player, mut moving) in (&players, &mut movings).iter() {
        //             if player.get_player() == player_evt {
        //                 moving.get_mut_velocity().y = amount as Coord * self.speed;
        //             }
        //         }
        //     }
        //     ControlToPlayer::Down(amount, player_evt) => {
        //         for (player, mut moving) in (&players, &mut movings).iter() {
        //             if player.get_player() == player_evt {
        //                 moving.get_mut_velocity().y = -amount as Coord * self.speed;
        //             }
        //         }
        //     }
        //     ControlToPlayer::Joy(x_opt, y_opt, player_evt) => {
        //         for (player, mut moving) in (&players, &mut movings).iter() {
        //             if player.get_player() == player_evt {
        //                 if let Some(x) = x_opt {
        //                     moving.get_mut_velocity().x = {
        //                         x as Coord * self.speed
        //                     };
        //                 }
        //                 if let Some(y) = y_opt {
        //                     moving.get_mut_velocity().y = {
        //                         y as Coord * self.speed
        //                     };
        //                 }
        //             }
        //         }
        //     }
        //     ControlToPlayer::A(player_evt) => {}
        //     ControlToPlayer::B(player_evt) => {}
        //     ControlToPlayer::X(player_evt) => {}
        //     ControlToPlayer::Y(player_evt) => {}
        //     ControlToPlayer::L1(player_evt) => {}
        //     ControlToPlayer::L2(player_evt) => {}
        //     ControlToPlayer::R1(player_evt) => {}
        //     ControlToPlayer::R2(player_evt) => {}
        // }
    }
}
