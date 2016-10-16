#[macro_use]
extern crate log;
extern crate env_logger;

extern crate core;

pub use core::crates::{components, event_enums, specs, systems, utils};
use core::crates::art::{game_3, game_6, make_square_render};
use core::crates::cgmath::{Euler, Point3, Rad, Vector3};
use core::crates::components::{Camera, CompMoving, CompPlayer, Gui, RenderData, Transform};
use core::crates::event::two_way_channel;
use core::crates::find_folder::Search;
use core::crates::graphics::load_texture;
use core::crates::rand::{Rng, thread_rng};
use core::crates::systems::{AiSystem, ControlSystem, FeederSystem, GuiSystem, MovingSystem, PlayerSystem, ScoreSystem};
use core::crates::utils::{Opter, OrthographicHelper, Player};

use std::str::FromStr;

mod player_system;

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
                "Game 6",
                OrthographicHelper::new(aspect_ratio, left, right, near, far),
                Box::new(|planner, back_event_clump, renderer, factory, ortho| {
        planner.mut_world()
            .create_now()
            .with(Camera::new(Point3::new(0.0, 0.0, 2.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), ortho, true))
            .build();

        let packet = make_square_render();

        let assets = Search::ParentsThenKids(5, 3)
            .for_folder("assets")
            .unwrap_or_else(|err| panic!("Did you forget to make an assets folder? Err: {:?}", err));

        let main_render = {
            let texture = load_texture(factory, assets.join(game_3::main::NAME));
            renderer.add_render(factory, &packet, texture)
        };

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::One))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(game_3::layers::PLAYER, *game_3::main::DEFAULT_TINT, game_3::main::PLAYER_1_STAND, game_3::main::SIZE))
            .build();

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::Two))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(game_3::layers::PLAYER, [1.0, 0.0, 0.0, 1.0], game_3::main::PLAYER_1_STAND, game_3::main::SIZE))
            .build();

        let selected = planner.mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(game_6::layers::GUI, game_6::main::DEFAULT_TINT.clone(), game_6::main::BOX, game_6::main::SIZE))
            .with(Gui::new(None, None, None, None))
            .build();


        let (score_to_feeder_front_channel, score_to_feeder_back_channel) = two_way_channel();

        planner.add_system(ScoreSystem::new(score_to_feeder_front_channel), "score", 60);

        let (feeder_to_ai_front_channel, feeder_to_ai_back_channel) = two_way_channel();

        planner.add_system(FeederSystem::new(feeder_to_ai_front_channel,
                                             score_to_feeder_back_channel,
                                             (|player, _score_1, _score_2| {
                                                 match player {
                                                     Player::One => vec![(Player::One, 0), (Player::Two, 0)],
                                                     Player::Two => vec![(Player::One, 0), (Player::Two, 0)],
                                                 }
                                             }),
                                             (|_score_1, _score_2| vec![(Player::One, 0), (Player::Two, 0)])),
                           "feeder",
                           50);

        let (ai_to_control_front_channel, ai_to_control_back_channel) = two_way_channel();

        planner.add_system(AiSystem::new(back_event_clump.take_ai()
                                             .unwrap_or_else(|| panic!("Ai was none")),
                                         feeder_to_ai_back_channel,
                                         ai_to_control_front_channel,
                                         vec![4, 9, 15, 9, 5, 2],
                                         vec![4, 8, 15, 9, 5, 2],
                                         16,
                                         -1.0,
                                         1.0,
                                         -1.0,
                                         1.0,
                                         Box::new(|| {
            if thread_rng().gen_range(0, 20) == 0 {
                thread_rng().choose(vec![-1.0, 1.0].as_slice()).unwrap_or_else(|| panic!("Choose was none")) * thread_rng().gen_range(0.5, 2.0)
            } else {
                1.0
            }
        }),
                                         Box::new(|| {
            if thread_rng().gen_range(0, 20) == 0 {
                thread_rng().gen_range(-1.0, 1.0)
            } else {
                0.0
            }
        })),
                           "ai",
                           40);

        let (control_to_player_front_channel, control_to_player_back_channel) = two_way_channel();
        let (control_to_gui_front_channel, control_to_gui_back_channel) = two_way_channel();

        planner.add_system(ControlSystem::new(back_event_clump.take_control()
                                                  .unwrap_or_else(|| panic!("Control was none")),
                                              ai_to_control_back_channel,
                                              control_to_player_front_channel,
                                              control_to_gui_front_channel),
                           "control",
                           30);

        planner.add_system(GuiSystem::new(selected, control_to_gui_back_channel), "gui", 25);

        planner.add_system(PlayerSystem::new(control_to_player_back_channel, 5.0, (|me, run_arg| player_system::basic_all_dir(me, run_arg))), "player", 20);

        planner.add_system(MovingSystem::new(), "moving", 15);
    }),
                Box::new(|planner, back_event_clump| {
        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::One))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .build();

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::Two))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .build();

        let selected = planner.mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(Gui::new(None, None, None, None))
            .build();


        let (score_to_feeder_front_channel, score_to_feeder_back_channel) = two_way_channel();

        planner.add_system(ScoreSystem::new(score_to_feeder_front_channel), "score", 60);

        let (feeder_to_ai_front_channel, feeder_to_ai_back_channel) = two_way_channel();

        planner.add_system(FeederSystem::new(feeder_to_ai_front_channel,
                                             score_to_feeder_back_channel,
                                             (|player, _score_1, _score_2| {
                                                 match player {
                                                     Player::One => vec![(Player::One, 0), (Player::Two, 0)],
                                                     Player::Two => vec![(Player::One, 0), (Player::Two, 0)],
                                                 }
                                             }),
                                             (|_score_1, _score_2| vec![(Player::One, 0), (Player::Two, 0)])),
                           "feeder",
                           50);

        let (ai_to_control_front_channel, ai_to_control_back_channel) = two_way_channel();

        planner.add_system(AiSystem::new(back_event_clump.take_ai()
                                             .unwrap_or_else(|| panic!("Ai was none")),
                                         feeder_to_ai_back_channel,
                                         ai_to_control_front_channel,
                                         vec![4, 9, 15, 9, 5, 2],
                                         vec![4, 8, 15, 9, 5, 2],
                                         16,
                                         -1.0,
                                         1.0,
                                         -1.0,
                                         1.0,
                                         Box::new(|| {
            if thread_rng().gen_range(0, 20) == 0 {
                thread_rng().choose(vec![-1.0, 1.0].as_slice()).unwrap_or_else(|| panic!("Choose was none")) * thread_rng().gen_range(0.5, 2.0)
            } else {
                1.0
            }
        }),
                                         Box::new(|| {
            if thread_rng().gen_range(0, 20) == 0 {
                thread_rng().gen_range(-1.0, 1.0)
            } else {
                0.0
            }
        })),
                           "ai",
                           40);

        let (control_to_player_front_channel, control_to_player_back_channel) = two_way_channel();
        let (control_to_gui_front_channel, control_to_gui_back_channel) = two_way_channel();

        planner.add_system(ControlSystem::new(back_event_clump.take_control()
                                                  .unwrap_or_else(|| panic!("Control was none")),
                                              ai_to_control_back_channel,
                                              control_to_player_front_channel,
                                              control_to_gui_front_channel),
                           "control",
                           30);

        planner.add_system(GuiSystem::new(selected, control_to_gui_back_channel), "gui", 25);

        planner.add_system(PlayerSystem::new(control_to_player_back_channel, 5.0, (|me, run_arg| player_system::basic_all_dir(me, run_arg))), "player", 20);

        planner.add_system(MovingSystem::new(), "moving", 15);
    }));
}
