#[macro_use]
extern crate log;
extern crate env_logger;

extern crate core;

use core::crates::utils::Opter;

use std::str::FromStr;

fn main() {
    env_logger::init().unwrap_or_else(|err| panic!("Unable to Initate Env Logger: {}", err));

    let opter = Opter::new();

    let t = opter.get_t();

    let g = opter.get_g();

    debug!("Getting Delta Time");

    let delta_time = {
        match t {
            Some(t) => {
                match f64::from_str(t) {
                    Ok(delta_time) => Some(delta_time),
                    Err(_) => None,
                }
            }
            None => None,
        }
    };

    start(g, delta_time);

    debug!("Game exited Successfully");
}

#[cfg(graphics = "not_sdl2")]
fn start(graphics_type: Option<String>, delta_time: Option<f64>) {
    match graphics_type {
        GraphicsType::Glutin => core::start_glutin(delta_time),
        GraphicsType::None => core::start_no_render(delta_time),
    }
}

#[cfg(graphics = "all")]
fn start(graphics_type: Option<String>, delta_time: Option<f64>) {
    match graphics_type {
        GraphicsType::Sdl2 => core::start_sdl2(delta_time),
        GraphicsType::Glutin => core::start_glutin(delta_time),
        GraphicsType::None => core::start_no_render(delta_time),
    }
}

#[cfg(graphics = "none")]
fn start(_: Option<String>, delta_time: Option<f64>) {
    core::start_no_render(delta_time)
}

#[cfg(graphics = "glutin")]
fn start(_: Option<String>, delta_time: Option<f64>) {
    core::start_glutin(delta_time)
}

#[cfg(graphics = "sdl2")]
fn start(_: Option<String>, delta_time: Option<f64>) {
    core::start_sdl2(delta_time)
}
