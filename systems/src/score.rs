use cgmath::{MetricSpace, Vector3};
use components::{CompMoving, CompPlayer, Transform};
use event::FrontChannel;
use event_enums::score_x_feeder::{ScoreFromFeeder, ScoreToFeeder};
use rand::{Rng, thread_rng};
use specs::{Join, RunArg, System};
use utils::{Coord, Delta, Player};

pub struct ScoreSystem {
    feeder_front_channel: FrontChannel<ScoreToFeeder<f64>, ScoreFromFeeder>,
    time: f64,
}

impl ScoreSystem {
    pub fn new(feeder_front_channel: FrontChannel<ScoreToFeeder<f64>, ScoreFromFeeder>) -> ScoreSystem {
        ScoreSystem {
            feeder_front_channel: feeder_front_channel,
            time: 0.0,
        }
    }
}

impl System<Delta> for ScoreSystem {
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
                *moving.get_mut_velocity() = STARTING_VELOCITY;
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
                    *moving.get_mut_velocity() = STARTING_VELOCITY;
                }
            }
        }
    }
}
