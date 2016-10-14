#[macro_use]
extern crate log;
extern crate env_logger;

extern crate core;

use core::crates::utils::{Player, Opter, OrthographicHelper};
use core::crates::cgmath::{Point3, Vector3, Rad, Euler};
use core::crates::systems::{MovingSystem, ScoreSystem, FeederSystem, AiSystem, ControlSystem,
                            PlayerSystem};
use core::crates::components::{RenderData, Transform, CompPlayer, CompMoving, Camera};
use core::crates::art::{make_square_render, game_3};
use core::crates::find_folder::Search;
use core::crates::graphics::load_texture;
use core::crates::event::two_way_channel;

use std::str::FromStr;

fn main() {
    env_logger::init().unwrap_or_else(|err| panic!("Unable to Initate Env Logger: {}", err));

    let opter = Opter::new();

    let t = opter.get_t();

    let g = opter.get_g();

    let delta_time = {
        match t {
            Some(t) => {
                match f64::from_str(t) {
                    Ok(delta_time) => Some(delta_time),
                    Err(_err) => None,
                }
            }
            None => None,
        }
    };

    let width = 640;
    let height = 640;

    let aspect_ratio = width as f32 / height as f32;

    let left = -10.0;
    let right = 10.0;
    let near = 0.0;
    let far = 10.0;

    core::start(delta_time,
                g,
                (width, height),
                "Game 5",
                OrthographicHelper::new(aspect_ratio, left, right, near, far),
                Box::new(|planner, back_event_clump, renderer, factory, ortho| {
        planner.mut_world()
            .create_now()
            .with(Camera::new(Point3::new(0.0, 0.0, 2.0),
                              Point3::new(0.0, 0.0, 0.0),
                              Vector3::new(0.0, 1.0, 0.0),
                              ortho,
                              true))
            .build();

        let packet = make_square_render();

        let assets =
            Search::ParentsThenKids(5, 3).for_folder("assets").unwrap_or_else(|err| panic!(err));

        let main_render = {
            let texture = load_texture(factory, assets.join(game_3::main::NAME));
            renderer.add_render(factory, &packet, texture)
        };

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::One))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                 Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                                 Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(game_3::layers::PLAYER,
                                  *game_3::main::DEFAULT_TINT,
                                  game_3::main::PLAYER_1_STAND,
                                  game_3::main::SIZE))
            .build();

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::Two))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                 Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                                 Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(game_3::layers::PLAYER,
                                  [1.0, 0.0, 0.0, 1.0],
                                  game_3::main::PLAYER_1_STAND,
                                  game_3::main::SIZE))
            .build();


        let (score_to_feeder_front_channel, score_to_feeder_back_channel) = two_way_channel();

        planner.add_system(ScoreSystem::new(score_to_feeder_front_channel), "score", 60);

        let (feeder_to_ai_front_channel, feeder_to_ai_back_channel) = two_way_channel();

        planner.add_system(FeederSystem::new(feeder_to_ai_front_channel,
                                             score_to_feeder_back_channel),
                           "feeder",
                           50);

        let (ai_to_control_front_channel, ai_to_control_back_channel) = two_way_channel();

        planner.add_system(AiSystem::new(back_event_clump.take_ai()
                                             .unwrap_or_else(|| panic!("Ai was none")),
                                         feeder_to_ai_back_channel,
                                         ai_to_control_front_channel),
                           "ai",
                           40);

        let (control_to_player_front_channel, control_to_player_back_channel) = two_way_channel();

        planner.add_system(ControlSystem::new(back_event_clump.take_control()
                                                  .unwrap_or_else(|| panic!("Control was none")),
                                              ai_to_control_back_channel,
                                              control_to_player_front_channel),
                           "control",
                           30);

        planner.add_system(PlayerSystem::new(control_to_player_back_channel),
                           "player",
                           20);

        planner.add_system(MovingSystem::new(), "moving", 15);
    }),
                Box::new(|planner, back_event_clump| {
        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::One))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                 Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                                 Vector3::new(1.0, 1.0, 1.0)))
            .build();

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::Two))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                 Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                                 Vector3::new(1.0, 1.0, 1.0)))
            .build();


        let (score_to_feeder_front_channel, score_to_feeder_back_channel) = two_way_channel();

        planner.add_system(ScoreSystem::new(score_to_feeder_front_channel), "score", 60);

        let (feeder_to_ai_front_channel, feeder_to_ai_back_channel) = two_way_channel();

        planner.add_system(FeederSystem::new(feeder_to_ai_front_channel,
                                             score_to_feeder_back_channel),
                           "feeder",
                           50);

        let (ai_to_control_front_channel, ai_to_control_back_channel) = two_way_channel();

        planner.add_system(AiSystem::new(back_event_clump.take_ai()
                                             .unwrap_or_else(|| panic!("Ai was none")),
                                         feeder_to_ai_back_channel,
                                         ai_to_control_front_channel),
                           "ai",
                           40);

        let (control_to_player_front_channel, control_to_player_back_channel) = two_way_channel();

        planner.add_system(ControlSystem::new(back_event_clump.take_control()
                                                  .unwrap_or_else(|| panic!("Control was none")),
                                              ai_to_control_back_channel,
                                              control_to_player_front_channel),
                           "control",
                           30);

        planner.add_system(PlayerSystem::new(control_to_player_back_channel),
                           "player",
                           20);

        planner.add_system(MovingSystem::new(), "moving", 15);
    }));
}
