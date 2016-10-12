
use components::{Camera, CompMoving, CompPlayer, RenderData, RenderId, Transform};
use art::make_square_render;
use art::game_3::{main, layers};
use specs::{Planner, World};
use time::precise_time_ns;
use cgmath::{Rad, Euler, Point3, Vector3};
use find_folder::Search;
use graphics::{OutColor, OutDepth, GlFactory, load_texture};
use utils::{OrthographicHelper, Delta, FpsCounter, Player};
use event::{BackChannel, two_way_channel};
use event_enums::main_x_game::{MainToGame, MainFromGame};

use systems::{AiSystem, FeederSystem, PlayerSystem, RenderSystem, ControlSystem, MovingSystem,
              ScoreSystem};
use systems::score::STARTING_VELOCITY;

use event_clump::BackEventClump;

pub struct Game {
    planner: Planner<Delta>,
    last_time: u64,
    fps_counter: FpsCounter,
    main_back_channel: BackChannel<MainToGame, MainFromGame>,
    delta: Option<f64>,
}

impl Game {
    pub fn new_no_render(mut back_event_clump: BackEventClump, fixed_delta: Option<f64>) -> Game {
        let mut planner = {
            let mut world = World::new();

            world.register::<Camera>();
            world.register::<CompMoving>();
            world.register::<CompPlayer>();
            world.register::<RenderData>();
            world.register::<RenderId>();
            world.register::<Transform>();

            Planner::<Delta>::new(world, 8)
        };

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(STARTING_VELOCITY))
            .with(CompPlayer::new(Player::One))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                 Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                                 Vector3::new(1.0, 1.0, 1.0)))
            .build();

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(STARTING_VELOCITY))
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

        Game {
            planner: planner,
            last_time: precise_time_ns(),
            fps_counter: FpsCounter::new(),
            main_back_channel: back_event_clump.take_game()
                .unwrap_or_else(|| panic!("Game was none")),
            delta: fixed_delta,
        }
    }

    pub fn new(factory: &mut GlFactory,
               mut back_event_clump: BackEventClump,
               ortho_helper: OrthographicHelper,
               out_color: OutColor,
               out_depth: OutDepth,
               fixed_delta: Option<f64>)
               -> Game {
        let mut planner = {
            let mut world = World::new();

            world.register::<Camera>();
            world.register::<CompMoving>();
            world.register::<CompPlayer>();
            world.register::<RenderData>();
            world.register::<RenderId>();
            world.register::<Transform>();

            Planner::<Delta>::new(world, 8)
        };

        let mut renderer = RenderSystem::new(back_event_clump.take_render()
                                                 .unwrap_or_else(|| panic!("Render was none")),
                                             out_color,
                                             out_depth);

        planner.mut_world()
            .create_now()
            .with(Camera::new(Point3::new(0.0, 0.0, 2.0),
                              Point3::new(0.0, 0.0, 0.0),
                              Vector3::new(0.0, 1.0, 0.0),
                              ortho_helper,
                              true))
            .build();

        let packet = make_square_render();

        let assets =
            Search::ParentsThenKids(5, 3).for_folder("assets").unwrap_or_else(|err| panic!(err));

        let main_render = {
            let texture = load_texture(factory, assets.join(main::NAME));
            renderer.add_render(factory, &packet, texture)
        };

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(STARTING_VELOCITY))
            .with(CompPlayer::new(Player::One))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                 Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                                 Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(layers::PLAYER,
                                  *main::DEFAULT_TINT,
                                  main::PLAYER_1_STAND,
                                  main::SIZE))
            .build();

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(STARTING_VELOCITY))
            .with(CompPlayer::new(Player::Two))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0),
                                 Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                                 Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(layers::PLAYER,
                                  [1.0, 0.0, 0.0, 1.0],
                                  main::PLAYER_1_STAND,
                                  main::SIZE))
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

        planner.add_system(renderer, "renderer", 10);

        Game {
            planner: planner,
            last_time: precise_time_ns(),
            fps_counter: FpsCounter::new(),
            main_back_channel: back_event_clump.take_game()
                .unwrap_or_else(|| panic!("Game was none")),
            delta: fixed_delta,
        }
    }

    pub fn frame(&mut self) -> bool {
        let delta = match self.delta {
            Some(delta) => delta,
            None => {
                let new_time = precise_time_ns();
                let delta = (new_time - self.last_time) as Delta / 1e9;
                self.last_time = new_time;
                delta
            }
        };

        self.planner.dispatch(delta);
        self.fps_counter.frame(delta);

        while let Some(event) = self.main_back_channel.try_recv_to() {
            match event {
                MainToGame::Exit => return false,
            }
        }

        true
    }
}
