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

    debug!("Getting Graphics");

    match g {
        Some(g_string) => {
            if g_string.contains("glutin") {
                core::start_glutin(delta_time);
            } else if g_string.contains("sdl") {
                core::start_sdl2(delta_time);
            } else {
                core::start_no_render(delta_time);
            }
        }
        None => core::start_no_render(delta_time),
    }
    core::start_sdl2(delta_time);
    debug!("Game exited Successfully");
}
