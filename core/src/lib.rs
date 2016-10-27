#[macro_use]
extern crate log;
extern crate time;
extern crate art;
extern crate systems;

// pub mod crates {
//     pub use ::{art, systems, time};
//     pub use art::crates;
//     pub use systems::crates::{cgmath, components, event, event_enums, find_folder, getopts, gfx, gfx_device_gl, graphics, image, neural, rand, rustc_serialize, specs, utils};
//     #[cfg(feature = "g_glutin")]
//     pub use systems::crates::{gfx_window_glutin, glutin};
//     #[cfg(feature = "g_sdl2")]
//     pub use systems::crates::{gfx_window_sdl, sdl2};
// }

// pub use crates::{cgmath, components, event, event_enums, find_folder, gfx, graphics, specs, utils};
// #[cfg(feature = "g_glutin")]
// pub use crates::glutin;
// #[cfg(feature = "g_sdl2")]
// pub use crates::sdl2;

mod event_clump;
mod game;
mod handle_events;

use event_clump::{BackEventClump, make_event_clumps};
use event_enums::main_x_ai::{MainFromAi, MainToAi};
use event_enums::main_x_control::MainFromControl;
#[allow(unused_imports)]
use event_enums::main_x_game::MainToGame;
#[allow(unused_imports)]
use event_enums::main_x_render::{MainFromRender, MainToRender};
use game::Game;
#[allow(unused_imports)]
use gfx::Device;
#[allow(unused_imports)]
use graphics::{GlEncoder, GlFactory};

use specs::Planner;
use std::thread;

use systems::render::RenderSystem;
use utils::Delta;

use utils::OrthographicHelper;

pub type Setup = Box<Fn(&mut Planner<Delta>, &mut BackEventClump, &mut RenderSystem, &mut GlFactory, OrthographicHelper)>;
pub type SetupNoRender = Box<Fn(&mut Planner<Delta>, &mut BackEventClump)>;

#[cfg(all(feature = "g_glutin", feature = "g_sdl2"))]
pub fn start<O>(delta_time: Option<f64>, g_string: Option<&String>, screen_size: (u32, u32), title: &str, camera: O, setup: Setup, setup_no_render: SetupNoRender)
    where O: AsRef<OrthographicHelper>
{
    match g_string {
        Some(g_string) => {
            if g_string.contains("glutin") {
                start_window(setup, delta_time, screen_size, title, camera);
            } else if g_string.contains("sdl2") {
                start_window(setup, delta_time, screen_size, title, camera);
            } else {
                start_no_render(setup_no_render, delta_time);
            }
        }
        None => start_no_render(setup_no_render, delta_time),
    }
}

#[cfg(all(not(feature = "g_sdl2"),feature = "g_glutin"))]
pub fn start<O>(delta_time: Option<f64>, g_string: Option<&String>, screen_size: (u32, u32), title: &str, camera: O, setup: Setup, setup_no_render: SetupNoRender)
    where O: AsRef<OrthographicHelper>
{
    match g_string {
        Some(g_string) => {
            if g_string.contains("glutin") {
                start_window(setup, delta_time, screen_size, title, camera);
            } else {
                start_no_render(setup_no_render, delta_time);
            }
        }
        None => start_no_render(setup_no_render, delta_time),
    }
}

#[cfg(all(feature = "g_sdl2", not(feature = "g_glutin")))]
pub fn start<O>(delta_time: Option<f64>, g_string: Option<&String>, screen_size: (u32, u32), title: &str, camera: O, setup: Setup, setup_no_render: SetupNoRender)
    where O: AsRef<OrthographicHelper>
{
    match g_string {
        Some(g_string) => {
            if g_string.contains("sdl2") {
                start_window(setup, delta_time, screen_size, title, camera);
            } else {
                start_no_render(setup_no_render, delta_time);
            }
        }
        None => start_no_render(setup_no_render, delta_time),
    }
}

#[cfg(all(not(feature = "g_sdl2"), not(feature = "g_glutin")))]
pub fn start<O>(delta_time: Option<f64>, _g: Option<&String>, _size: (u32, u32), _title: &str, _ortho: O, _setup: Setup, setup_no_render: SetupNoRender)
    where O: AsRef<OrthographicHelper>
{
    start_no_render(setup_no_render, delta_time);
}

fn start_no_render(setup: SetupNoRender, fixed_delta: Option<f64>) {
    let (mut front_event_clump, back_event_clump) = make_event_clumps();

    let game = Game::new_no_render(setup, back_event_clump, fixed_delta);

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

#[cfg(any(feature = "g_glutin", feature = "g_sdl2"))]
fn start_window<O>(setup: Setup, fixed_delta: Option<f64>, screen_size: (u32, u32), title: &str, ortho: O)
    where O: AsRef<OrthographicHelper>
{
    #[cfg(feature = "g_glutin")]
    use graphics::rl_glutin::build_window;
    #[cfg(feature = "g_glutin")]
    use handle_events::glutin::handle_events;

    #[cfg(all(feature = "g_sdl2", not(feature = "g_glutin")))]
    use graphics::rl_sdl2::build_window;
    #[cfg(all(feature = "g_sdl2", not(feature = "g_glutin")))]
    use handle_events::sdl2::handle_events;

    let (mut front_event_clump, back_event_clump) = make_event_clumps();

    let (width, height): (u32, u32) = screen_size;

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

    let game = Game::new(setup, gfx_window.get_mut_factory(), back_event_clump, ortho.as_ref().clone(), out_color, out_depth, fixed_delta);

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
