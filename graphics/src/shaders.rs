use find_folder::Search;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Shaders {
    vertex: Vec<u8>,
    fragment: Vec<u8>,
}

impl Shaders {
    pub fn new_from_bytes(vertex: &[u8], fragment: &[u8]) -> Shaders {
        let mut v_vec = vec![];
        let mut f_vec = vec![];

        v_vec.extend_from_slice(vertex);
        f_vec.extend_from_slice(fragment);

        Shaders {
            vertex: v_vec,
            fragment: f_vec,
        }
    }

    pub fn new(vertex_name: &'static str, fragment_name: &'static str) -> Shaders {
        let shaders_path = match Search::ParentsThenKids(3, 3).for_folder("shaders") {
            Ok(shaders_path) => shaders_path,
            Err(err) => panic!("find folder shaders error: {}", err),
        };

        let mut vertex_path = shaders_path.clone();
        let mut fragment_path = shaders_path.clone();

        vertex_path.push(vertex_name);
        fragment_path.push(fragment_name);

        let vertex_file = match File::open(vertex_path) {
            Ok(file) => file,
            Err(err) => panic!("vertex file open err: {}", err),
        };
        let fragment_file = match File::open(fragment_path) {
            Ok(file) => file,
            Err(err) => panic!("fragment file open err: {}", err),
        };

        let mut vertex_reader = BufReader::new(vertex_file);
        let mut fragment_reader = BufReader::new(fragment_file);

        let mut vertex_buffer = vec![];
        let mut fragment_buffer = vec![];

        match vertex_reader.read_to_end(&mut vertex_buffer) {
            Ok(_) => (),
            Err(err) => panic!("vertex reader read to end error: {}", err),
        };
        match fragment_reader.read_to_end(&mut fragment_buffer) {
            Ok(_) => (),
            Err(err) => panic!("fragment reader read to end error: {}", err),
        };

        Shaders {
            vertex: vertex_buffer,
            fragment: fragment_buffer,
        }
    }

    pub fn get_vertex_shader(&self) -> &[u8] {
        self.vertex.as_slice()
    }

    pub fn get_fragment_shader(&self) -> &[u8] {
        self.fragment.as_slice()
    }
}
