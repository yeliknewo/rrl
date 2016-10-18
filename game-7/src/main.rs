#[macro_use]
extern crate log;
extern crate env_logger;

extern crate core;

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

    core::start(delta_time, g, (width, height), "Game 7", OrthographicHelper::new(aspect_ratio, left, right, near, far), Box::new(|planner, back_event_clump, renderer, factory, ortho| {

    });
}
