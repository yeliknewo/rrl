#[macro_use]
extern crate log;
extern crate env_logger;

extern crate core;

fn main() {
    env_logger::init().unwrap_or_else(|err| panic!("Unable to Initate Env Logger: {}", err));

    core::start_sdl2();
    warn!("Game exited Successfully");
}
