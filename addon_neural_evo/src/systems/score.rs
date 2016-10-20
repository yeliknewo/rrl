use base_comps::{CompMoving, CompPlayer, Transform};
use base_events::score_x_feeder::{ScoreFromFeeder, ScoreToFeeder};
use cgmath::{MetricSpace, Vector3};
use event_core::duo_channel::DuoChannel;
use events::{FromScore, ToScore};
use rand::{Rng, thread_rng};
use specs::{Join, RunArg, System};
use utils::{Coord, Delta, Player};

type SendEvent = Box<From<FromScore>>;
type RecvEvent = Box<Into<ToScore>>;

pub struct ScoreSystem<ID: Eq> {
    // feeder_front_channel: FrontChannel<ScoreToFeeder<f64>, ScoreFromFeeder>,
    feeder_channel_id: ID,
    channels: Vec<DuoChannel<ID, SendEvent, RecvEvent>>,
    time: f64,
}

impl<ID> ScoreSystem<ID>
    where ID: Eq
{
    pub fn new(channels: Vec<DuoChannel<ID, SendEvent, RecvEvent>>, feeder_channel_id: ID) -> ScoreSystem<ID> {
        ScoreSystem {
            feeder_channel_id: feeder_channel_id,
            channels: channels,
            // feeder_front_channel: feeder_front_channel,
            time: 0.0,
        }
    }
}

impl<ID> System<Delta> for ScoreSystem<ID>
    where ID: Eq
{
    fn run(&mut self, args: RunArg, delta_time: Delta) {
        self.time += delta_time;

        let (players, mut transforms, mut movings) = args.fetch(|w| (w.read::<CompPlayer>(), w.write::<Transform>(), w.write::<CompMoving>()));

        let mut done = false;

        let mut player_info: Vec<(Player, Vector3<Coord>)> = vec![];

        for (player, transform) in (&players, &transforms).iter() {
            let pos = transform.get_pos();
            player_info.push((player.get_player(), pos));
            if pos.x.abs() > 10.0 || pos.y.abs() > 10.0 {
                if player.get_player() == Player::One {
                    continue;
                }
                self.feeder_front_channel
                    .send_to(ScoreToFeeder::Lose(player.get_player(), 0.0, -150.0));
                done = true;
                break;
            }
        }

        if self.time > 30.0 {
            self.feeder_front_channel.send_to(ScoreToFeeder::LoseBoth(-300.0, -300.0));
            done = true;
        }

        if done {
            self.time = 0.0;
            for (player, mut transform, mut moving) in (&players, &mut transforms, &mut movings).iter() {
                transform.set_pos(match player.get_player() {
                    Player::One => Vector3::new(thread_rng().gen_range(-9.0, 9.0), thread_rng().gen_range(-9.0, 9.0), 0.0),
                    Player::Two => Vector3::new(thread_rng().gen_range(-9.0, 9.0), thread_rng().gen_range(-9.0, 9.0), 0.0),
                });
                *moving.get_mut_velocity() = Vector3::new(0.0, 0.0, 0.0);
            }
        } else {
            let my_player = player_info.get(0).unwrap_or_else(|| panic!("Panic")).0;
            let my_pos = player_info.get(0).unwrap_or_else(|| panic!("Panic")).1;

            let other_player = player_info.get(1).unwrap_or_else(|| panic!("Panic")).0;
            let other_pos = player_info.get(1).unwrap_or_else(|| panic!("Panic")).1;

            match my_player {
                Player::One => {
                    if my_pos.distance(other_pos) < 1.0 {
                        self.feeder_front_channel
                            .send_to(ScoreToFeeder::Lose(other_player, 300.0, -60.0));
                        done = true;
                    }
                }
                Player::Two => {}
            }

            match other_player {
                Player::One => {
                    if other_pos.distance(my_pos) < 1.0 {
                        self.feeder_front_channel
                            .send_to(ScoreToFeeder::Lose(my_player, 300.0, -60.0));
                        done = true;
                    }
                }
                Player::Two => {}
            }
            if done {
                self.time = 0.0;
                for (player, mut transform, mut moving) in (&players, &mut transforms, &mut movings).iter() {
                    transform.set_pos(match player.get_player() {
                        Player::One => Vector3::new(thread_rng().gen_range(-9.0, 9.0), thread_rng().gen_range(-9.0, 9.0), 0.0),
                        Player::Two => Vector3::new(thread_rng().gen_range(-9.0, 9.0), thread_rng().gen_range(-9.0, 9.0), 0.0),
                    });
                    *moving.get_mut_velocity() = Vector3::new(0.0, 0.0, 0.0);
                }
            }
        }
    }
}
