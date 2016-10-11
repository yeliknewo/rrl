use std::collections::HashMap;
use sdl2::event::EventType;
use sdl2::video::GLContext;
use sdl2::controller::GameController;
use sdl2::{EventPump, GameControllerSubsystem};
use sdl2;
use gfx_window_sdl;
use utils::Player;

use super::{WindowSettings, GfxWindow};

pub type Window = sdl2::video::Window;
pub type Extras = (GLContext,
                   Option<EventPump>,
                   Option<GameControllerSubsystem>,
                   HashMap<i32, (GameController, Player)>);

pub fn build_window(window_settings: WindowSettings) -> GfxWindow<Window, Extras> {
    let sdl = sdl2::init().unwrap_or_else(|err| panic!("Error while sdl2::init: {:?}", err));

    let video = sdl.video()
        .unwrap_or_else(|err| panic!("Error while making sdl.video(): {:?}", err));

    let mut event_pump = sdl.event_pump()
        .unwrap_or_else(|err| panic!("Error while making event pump: {:?}", err));
    event_pump.enable_event(EventType::KeyDown);
    event_pump.enable_event(EventType::KeyUp);
    event_pump.enable_event(EventType::Window);
    event_pump.enable_event(EventType::ControllerAxisMotion);
    event_pump.enable_event(EventType::ControllerButtonDown);
    event_pump.enable_event(EventType::ControllerButtonUp);

    let controller_subsystem = sdl.game_controller()
        .unwrap_or_else(|err| panic!("Error while making controller sub system: {:?}", err));
    controller_subsystem.set_event_state(true);

    let gl_attr = video.gl_attr();
    gl_attr.set_context_version(3, 2);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    let (title, width, height) = window_settings;

    let mut builder = video.window(title, width, height);
    let (window, context, device, factory, out_color, out_depth) =
        gfx_window_sdl::init(&mut builder);

    GfxWindow::new(out_color,
                   out_depth,
                   device,
                   factory,
                   window,
                   (context, Some(event_pump), Some(controller_subsystem), HashMap::new()))
}
