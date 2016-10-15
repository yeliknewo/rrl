use components::{CompMoving, CompPlayer};
use event::BackChannel;
use event_enums::control_x_player::{ControlFromPlayer, ControlToPlayer};
use specs::{Allocator, Join, MaskedStorage, RunArg, Storage, System};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use utils::{Coord, Delta, Player};

pub struct PlayerSystem<F1, T1, T2, T3>
    where F1: Send + Fn(ControlToPlayer<f64>,
                 RunArg),
          T1: Deref<Target = Allocator>,
          T2: Deref<Target = MaskedStorage<CompPlayer>>,
          T3: DerefMut<Target = MaskedStorage<CompMoving>>
{
    control_back_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
    event_handler: F1,
    speed: Coord,
    t1_data: PhantomData<T1>,
    t2_data: PhantomData<T2>,
    t3_data: PhantomData<T3>,
}

impl<F1, T1, T2, T3> PlayerSystem<F1, T1, T2, T3>
    where F1: Send + Fn(ControlToPlayer<f64>,
                 &Storage<CompPlayer, T1, T2>,
                 &mut Storage<CompMoving, T1, T3>),
          T1: Deref<Target = Allocator>,
          T2: Deref<Target = MaskedStorage<CompPlayer>>,
          T3: DerefMut<Target = MaskedStorage<CompMoving>>
{
    pub fn new(control_back_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
               speed: Coord,
               event_handler: F1)
               -> PlayerSystem<F1, T1, T2, T3> {
        PlayerSystem {
            control_back_channel: control_back_channel,
            speed: speed,
            event_handler: event_handler,
            t1_data: PhantomData::default(),
            t2_data: PhantomData::default(),
            t3_data: PhantomData::default(),
        }
    }
}

impl<F1> System<Delta> for PlayerSystem<F1>
    where F1: Send + Fn(ControlToPlayer<f64>,
                 &Storage<CompPlayer, RwLockReadGuard<Allocator>, RwLockWriteGuard<MaskedStorage<CompPlayer>>>,
                 &mut Storage<CompMoving, RwLockReadGuard<Allocator>, RwLockWriteGuard<'a, MaskedStorage<CompMoving>>>)
{
    fn run(&mut self,
           arg: RunArg,
           _: Delta) {
        let (players, mut movings) = arg.fetch(|w| (w.read::<CompPlayer>(), w.write::<CompMoving>()));

        while let Some(event) = self.control_back_channel.try_recv_to() {
            (self.event_handler)(event,
                                 &players,
                                 &mut movings)
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
}
