#[macro_use]
extern crate log;
pub extern crate time;
pub extern crate art;
pub extern crate systems;

pub mod crates {
    pub use ::{art, time};
    pub use systems::crates::{gfx, graphics, utils, event_enums, getopts, components, event,
                              neural, gfx_device_gl, glutin, gfx_window_glutin, sdl2,
                              gfx_window_sdl, find_folder, image, cgmath, rustc_serialize, rand,
                              specs};
    pub use art::crates;
}

pub use crates::{components, event, gfx, graphics, utils, event_enums, specs, cgmath, find_folder,
                 sdl2, glutin};

mod event_clump;
mod game;
mod handle_events;

use std::thread;
use gfx::Device;
use graphics::GlEncoder;

use utils::OrthographicHelper;
use event_enums::main_x_ai::{MainToAi, MainFromAi};
use event_enums::main_x_control::MainFromControl;
use event_enums::main_x_render::{MainToRender, MainFromRender};
use event_enums::main_x_game::MainToGame;

use event_clump::make_event_clumps;
use game::Game;

pub fn start_no_render(fixed_delta: Option<f64>) {
    let (mut front_event_clump, back_event_clump) = make_event_clumps();

    let game = Game::new_no_render(back_event_clump, fixed_delta);

    thread::spawn(|| {
        let mut game = game;
        while game.frame() {
        }
    });

    'main: loop {
        if let Some(event) = front_event_clump.get_mut_control()
            .unwrap_or_else(|| panic!("Control was none"))
            .try_recv_from() {
            match event {
                MainFromControl::Save => {
                    front_event_clump.get_mut_ai()
                        .unwrap_or_else(|| panic!("Ai was none"))
                        .send_to(MainToAi::Save);
                    match front_event_clump.get_mut_ai()
                        .unwrap_or_else(|| panic!("Ai was none"))
                        .recv_from() {
                        MainFromAi::Saved => warn!("Autosaved"),
                    };
                }
            }
        }

        if let Some(event) = front_event_clump.get_mut_game()
            .unwrap_or_else(|| panic!("Game was none"))
            .try_recv_from() {
            match event {

            }
        }
    }
}

pub fn start_glutin(fixed_delta: Option<f64>) {
    use graphics::rl_glutin::build_window;
    use handle_events::glutin::handle_events;

    debug!("Starting Core Start");

    let (mut front_event_clump, back_event_clump) = make_event_clumps();

    let (width, height): (u32, u32) = (640, 640);

    let title = "rl-game-4";

    let left = -10.0;
    let right = 10.0;

    let near = 0.0;
    let far = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let ortho_helper = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    // warn!("Making Window");
    let mut gfx_window = build_window((title, width, height));

    // warn!("Making Encoder");
    let encoder: GlEncoder = gfx_window.get_mut_factory().create_command_buffer().into();

    {
        let mut render_event_core = front_event_clump.get_mut_render()
            .unwrap_or_else(|| panic!("Render was none"));

        // warn!("Sending Empty Encoder");
        render_event_core.send_to(MainToRender::Encoder(encoder.clone_empty()));
        // warn!("Sending Encoder");
        render_event_core.send_to(MainToRender::Encoder(encoder));
    }

    let out_color = gfx_window.get_out_color().clone();
    let out_depth = gfx_window.get_out_depth().clone();

    // warn!("Making Game");
    // let game = Game::new_no_render(back_event_clump);

    let game = Game::new(gfx_window.get_mut_factory(),
                         back_event_clump,
                         ortho_helper,
                         out_color,
                         out_depth,
                         fixed_delta);

    // warn!("Making Game Thread");
    let game_handle = thread::spawn(|| {
        let mut game = game;
        while game.frame() {
        }
    });

    'main: loop {
        // warn!("Main Loop");
        if let Some(event) = front_event_clump.get_mut_render()
            .unwrap_or_else(|| panic!("Render was none"))
            .try_recv_from() {
            match event {
                MainFromRender::Encoder(mut encoder) => {
                    if handle_events(&mut gfx_window, &mut front_event_clump) {
                        front_event_clump.get_mut_render()
                            .unwrap_or_else(|| panic!("Render was none"))
                            .send_to(MainToRender::Encoder(encoder));
                        break 'main;
                    }

                    encoder.flush(gfx_window.get_mut_device());
                    front_event_clump.get_mut_render()
                        .unwrap_or_else(|| panic!("Render was none"))
                        .send_to(MainToRender::Encoder(encoder));
                    gfx_window.swap_buffers();
                    gfx_window.get_mut_device().cleanup();
                }
            }
        }

        if let Some(event) = front_event_clump.get_mut_control()
            .unwrap_or_else(|| panic!("Control was none"))
            .try_recv_from() {
            match event {
                MainFromControl::Save => {
                    front_event_clump.get_mut_ai()
                        .unwrap_or_else(|| panic!("Ai was none"))
                        .send_to(MainToAi::Save);
                    match front_event_clump.get_mut_ai()
                        .unwrap_or_else(|| panic!("Ai was none"))
                        .recv_from() {
                        MainFromAi::Saved => (),
                    };
                }
            }
        }

        if let Some(event) = front_event_clump.get_mut_game()
            .unwrap_or_else(|| panic!("Game was none"))
            .try_recv_from() {
            match event {

            }
        }
    }

    front_event_clump.get_mut_ai().unwrap_or_else(|| panic!("Ai was none")).send_to(MainToAi::Save);
    match front_event_clump.get_mut_ai().unwrap_or_else(|| panic!("Ai was none")).recv_from() {
        MainFromAi::Saved => {
            warn!("Tried to save, might have worked");
        }
    }
    front_event_clump.get_mut_game()
        .unwrap_or_else(|| panic!("Game was none"))
        .send_to(MainToGame::Exit);

    game_handle.join().unwrap_or_else(|err| panic!("Error: {:?}", err));
}

pub fn start_sdl2(fixed_delta: Option<f64>) {
    use graphics::rl_sdl2::build_window;
    use handle_events::sdl2::handle_events;

    debug!("Starting Core Start");

    let (mut front_event_clump, back_event_clump) = make_event_clumps();

    let (width, height): (u32, u32) = (640, 640);

    let title = "rl-game-4";

    let left = -10.0;
    let right = 10.0;

    let near = 0.0;
    let far = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let ortho_helper = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    // warn!("Making Window");
    let mut gfx_window = build_window((title, width, height));

    // warn!("Making Encoder");
    let encoder: GlEncoder = gfx_window.get_mut_factory().create_command_buffer().into();

    {
        let mut render_event_core = front_event_clump.get_mut_render()
            .unwrap_or_else(|| panic!("Render was none"));

        // warn!("Sending Empty Encoder");
        render_event_core.send_to(MainToRender::Encoder(encoder.clone_empty()));
        // warn!("Sending Encoder");
        render_event_core.send_to(MainToRender::Encoder(encoder));
    }

    let out_color = gfx_window.get_out_color().clone();
    let out_depth = gfx_window.get_out_depth().clone();

    // warn!("Making Game");
    // let game = Game::new_no_render(back_event_clump);

    let game = Game::new(gfx_window.get_mut_factory(),
                         back_event_clump,
                         ortho_helper,
                         out_color,
                         out_depth,
                         fixed_delta);

    // warn!("Making Game Thread");
    let game_handle = thread::spawn(|| {
        let mut game = game;
        while game.frame() {
        }
    });

    'main: loop {
        // warn!("Main Loop");
        if let Some(event) = front_event_clump.get_mut_render()
            .unwrap_or_else(|| panic!("Render was none"))
            .try_recv_from() {
            match event {
                MainFromRender::Encoder(mut encoder) => {
                    if handle_events(&mut gfx_window, &mut front_event_clump) {
                        front_event_clump.get_mut_render()
                            .unwrap_or_else(|| panic!("Render was none"))
                            .send_to(MainToRender::Encoder(encoder));
                        break 'main;
                    }

                    encoder.flush(gfx_window.get_mut_device());
                    front_event_clump.get_mut_render()
                        .unwrap_or_else(|| panic!("Render was none"))
                        .send_to(MainToRender::Encoder(encoder));
                    gfx_window.swap_buffers();
                    gfx_window.get_mut_device().cleanup();
                }
            }
        }

        if let Some(event) = front_event_clump.get_mut_control()
            .unwrap_or_else(|| panic!("Control was none"))
            .try_recv_from() {
            match event {
                MainFromControl::Save => {
                    front_event_clump.get_mut_ai()
                        .unwrap_or_else(|| panic!("Ai was none"))
                        .send_to(MainToAi::Save);
                    match front_event_clump.get_mut_ai()
                        .unwrap_or_else(|| panic!("Ai was none"))
                        .recv_from() {
                        MainFromAi::Saved => (),
                    };
                }
            }
        }

        if let Some(event) = front_event_clump.get_mut_game()
            .unwrap_or_else(|| panic!("Game was none"))
            .try_recv_from() {
            match event {

            }
        }
    }

    front_event_clump.get_mut_ai().unwrap_or_else(|| panic!("Ai was none")).send_to(MainToAi::Save);
    match front_event_clump.get_mut_ai().unwrap_or_else(|| panic!("Ai was none")).recv_from() {
        MainFromAi::Saved => {
            warn!("Tried to save, might have worked");
        }
    }
    front_event_clump.get_mut_game()
        .unwrap_or_else(|| panic!("Game was none"))
        .send_to(MainToGame::Exit);

    game_handle.join().unwrap_or_else(|err| panic!("Error: {:?}", err));
}
