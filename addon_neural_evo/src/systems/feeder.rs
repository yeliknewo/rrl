use base_comps::{CompMoving, CompPlayer, Transform};
use base_events::score_x_feeder::{ScoreFromFeeder, ScoreToFeeder};
use cgmath::Vector3;
use event_core::duo_channel::DuoChannel;
use events::{FromAi, ToAi};
use events::{FromFeeder, ToFeeder};
use specs::{Join, RunArg, System};
use std::any::Any;
use utils::{Coord, Delta, Player};

// const DISTANCE_WEIGHT: f32 = 5.0;
// const TIME_WEIGHT: f64 = 1.0;

pub struct FeederSystem<ID: Eq, T: Send + Fn(Player, f64, f64) -> Vec<(Player, i64)>, F: Send + Fn(f64, f64) -> Vec<(Player, i64)>> {
    channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>,
    // ai_front_channel: FrontChannel<E1, E2>,
    // score_back_channel: BackChannel<ScoreToFeeder<f64>, ScoreFromFeeder>,
    one_player_lose: T,
    both_player_lose: F,
    time: f64,
}

impl<ID, T, F> FeederSystem<ID, T, F>
    where ID: Eq,
          T: Send + Fn(Player, f64, f64) -> Vec<(Player, i64)>,
          F: Send + Fn(f64, f64) -> Vec<(Player, i64)>
{
    pub fn new(channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>, one_player_lose: T, both_player_lose: F) -> FeederSystem<ID, T, F> {
        FeederSystem {
            channels: channels,
            time: 0.0,
            one_player_lose: one_player_lose,
            both_player_lose: both_player_lose,
        }
    }
}

impl<ID, T, F> System<Delta> for FeederSystem<ID, T, F>
    where ID: Send + Eq,
          T: Send + Fn(Player, f64, f64) -> Vec<(Player, i64)>,
          F: Send + Fn(f64, f64) -> Vec<(Player, i64)>
{
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        self.time += delta_time;

        let (transforms, players, movings) = arg.fetch(|w| (w.read::<Transform>(), w.read::<CompPlayer>(), w.read::<CompMoving>()));

        if let Some(event) = self.score_back_channel.try_recv_to() {
            match event {
                ScoreToFeeder::Lose(loser, score_1, score_2) => {
                    self.ai_front_channel.send_to(ToAi::RewardAndEnd({
                        (self.one_player_lose)(loser, score_1, score_2)
                    }));
                    self.time = 0.0;
                }
                ScoreToFeeder::LoseBoth(score_1, score_2) => {
                    self.ai_front_channel.send_to(ToAi::RewardAndEnd({
                        (self.both_player_lose)(score_1, score_2)
                    }));
                    self.time = 0.0;
                }
            }
        }

        let mut player_data = vec![];

        for (transform, player, moving) in (&transforms, &players, &movings).iter() {
            player_data.push((player.get_player(), transform.get_pos(), moving.get_velocity()));
        }

        self.ai_front_channel.send_to(ToAi::Reward(player_data.iter()
            .filter_map(|me| {
                let other = player_data.iter()
                    .filter_map(|other| {
                        if me.0 != other.0 && me.1 != other.1 {
                            Some(other.1)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Vector3<Coord>>>();
                if other.len() == 1 {
                    match me.0 {
                        Player::One => Some((me.0, 1)),//-me.1.distance(center) as i64)),
                        Player::Two => Some((me.0, 1)),//-me.1.distance(center) as i64)),
                    }
                } else {
                    None
                }
            })
            .collect()));

        let players = vec![Player::One, Player::Two];

        for player in players {
            let world_state: Vec<f64> = match player {
                Player::One => {
                    let p1_pos = player_data[0].1;
                    let p2_pos = player_data[1].1;

                    vec![p1_pos.x as f64, p1_pos.y as f64, p2_pos.x as f64, p2_pos.y as f64]//)dot1 as f64, mag as f64, dot2 as f64, mag2 as f64, vel.x as f64, vel.y as f64, self.time)
                }
                Player::Two => {
                    let p1_pos = player_data[0].1;
                    let p2_pos = player_data[1].1;

                    vec![p2_pos.x as f64, p2_pos.y as f64, p1_pos.x as f64, p1_pos.y as f64]//dot1 as f64, mag as f64, dot2 as f64, mag2 as f64, vel.x as f64, vel.y as f64, self.time)
                }
            };

            self.ai_front_channel.send_to(ToAi::WorldState(player, world_state));
        }
    }
}
