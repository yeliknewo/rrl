pub extern crate yaml_rust;

pub mod crates {
    pub use ::yaml_rust;
}

use std::fs::File;
use std::io::{self, BufReader, Read};

use yaml_rust::{Yaml, YamlLoader};
use yaml_rust::scanner::ScanError;

#[derive(Debug)]
pub struct Configger {
    graphics: GraphicsType,
    delta_time: Option<f64>,
}

impl Configger {
    pub fn new() -> Result<Configger, Error> {
        let path = "config.yaml";

        let f = try!(File::open(path).map_err(|err| Error::Io(err)));
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();
        let _ = try!(reader.read_to_string(&mut buffer).map_err(|err| Error::Io(err)));
        let docs = try!(YamlLoader::load_from_str(&buffer).map_err(|err| Error::Yaml(err)));

        let mut graphics = GraphicsType::None;

        let mut delta_time = None;

        {
            // match docs {
            match try!(docs.get(0).ok_or(Error::BadFile)) {
                &Yaml::Hash(ref hashes) => {
                    for hash in hashes {
                        match hash.0 {
                            &Yaml::String(ref key) => {
                                match key.as_str() {
                                    "graphics" => {
                                        match hash.1.as_str() {
                                            Some("sdl") | Some("sdl2") => graphics = GraphicsType::Sdl2,
                                            Some("glutin") => graphics = GraphicsType::Glutin,
                                            _ => graphics = GraphicsType::None,
                                        }
                                    }
                                    "delta_time" => delta_time = hash.1.as_f64(),
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
            // }
        }

        Ok(Configger {
            graphics: graphics,
            delta_time: delta_time,
        })
    }

    pub fn get_round_time_limit(&self) -> f64 {
        30.0
    }

    pub fn get_delta_time(&self) -> Option<f64> {
        self.delta_time.clone()
    }

    pub fn get_graphics_type(&self) -> &GraphicsType {
        &self.graphics
    }
}

#[derive(Debug)]
pub enum GraphicsType {
    Sdl2,
    Glutin,
    None,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Yaml(ScanError),
    BadFile,
}
