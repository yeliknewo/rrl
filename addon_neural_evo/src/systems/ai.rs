use event_core::duo_channel::DuoChannel;
use events::{FromAi, ToAi};
use find_folder::Search;
use neural::evolution::EvolutionaryTrainer;
use neural::network::NeuralNetwork;
use num::{Float, FromPrimitive, Num, ToPrimitive};
use rand::distributions::range::SampleRange;
use rustc_serialize::{Decodable, Encodable, json};
use specs::{RunArg, System};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::{Read, Write};
use std::path::PathBuf;
use utils::{Delta, Opter, Player};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Brain {
    Chase,
    Flee,
}

impl<'a> Brain {
    fn brain_to_name(brain: Brain) -> &'a str {
        match brain {
            Brain::Chase => "_chase",
            Brain::Flee => "_flee",
        }
    }

    // fn name_to_brain(name: &str) -> Option<Brain> {
    //     match name {
    //         "_chase" => Some(Brain::Chase),
    //         "_flee" => Some(Brain::Flee),
    //         _ => None,
    //     }
    // }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct BrainClump<S: Debug + Ord + Clone + Decodable + Encodable + ToPrimitive + Num, W: Any + Send + Float + SampleRange + FromPrimitive + Decodable + Encodable> {
    trainer: EvolutionaryTrainer<S, W>,
    player_mapper: HashMap<Player, usize>,
    used_indices: Vec<usize>,
    rewards: Vec<(usize, S)>,
    players: Vec<Player>,
}

impl<S: Debug + Ord + Clone + Decodable + Encodable + ToPrimitive + Num, W: Any + Send + Float + SampleRange + FromPrimitive + Decodable + Encodable> BrainClump<S, W> {
    fn new(network_count: usize, input_size: S, network_size: Vec<S>, min_weight: W, max_weight: W, min_bias: W, max_bias: W) -> BrainClump<S, W> {
        let mut networks = HashMap::new();

        for index in 0..network_count {
            networks.insert(index, NeuralNetwork::new_random(input_size.clone(), &network_size, min_weight, max_weight, min_bias, max_bias));
        }

        BrainClump {
            trainer: EvolutionaryTrainer::new(networks),
            player_mapper: HashMap::new(),
            used_indices: vec![],
            rewards: vec![],
            players: vec![],
        }
    }

    fn load(brain: Brain) -> Option<BrainClump<S, W>> {
        let load_path = match BrainClump::<S, W>::get_load_path(Brain::brain_to_name(brain)) {
            Some(path) => path,
            None => return None,
        };

        let f = match File::open(load_path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let mut reader = BufReader::new(f);

        let mut string = String::new();

        match reader.read_to_string(&mut string) {
            Ok(_) => (),
            Err(_) => return None,
        }

        let decoded = match json::decode(string.as_str()) {
            Ok(decoded) => decoded,
            Err(_) => return None,
        };

        decoded
    }

    fn get_load_path(name: &str) -> Option<PathBuf> {
        let mut saves = Search::ParentsThenKids(5, 5)
            .for_folder("networks")
            .unwrap_or_else(|err| panic!("Did you forget to make a networks folder?: {:?}", err));

        let mut filename = String::new();

        let opter = Opter::new();

        filename.push_str(match opter.get_r() {
            Some(string) => string.as_str(),
            None => return None,
        });
        filename.push_str(name);
        filename.push_str(".network");
        saves.push(filename);
        Some(saves)
    }

    fn get_save_path(name: &str) -> Option<PathBuf> {
        let mut saves = Search::ParentsThenKids(5, 5)
            .for_folder("networks")
            .unwrap_or_else(|err| panic!("{:?}", err));

        let mut filename = String::new();

        let opter = Opter::new();

        filename.push_str(match opter.get_w() {
            Some(string) => string.as_str(),
            None => return None,
        });
        filename.push_str(name);
        filename.push_str(".network");
        saves.push(filename);
        Some(saves)
    }

    fn save(&self, brain: Brain) {
        let save_path = match BrainClump::<S, W>::get_save_path(Brain::brain_to_name(brain)) {
            Some(path) => path,
            None => return,
        };

        let encoded = match json::encode(&self) {
            Ok(encoded) => encoded,
            Err(err) => {
                error!("Unable to Encode to save: {:?}", err);
                return;
            }
        };

        let f = File::create(save_path).unwrap_or_else(|err| panic!("{:?}", err));

        let mut writer = BufWriter::new(f);

        writer.write(encoded.as_bytes()).unwrap_or_else(|err| panic!("{:?}", err));
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
        self.players.sort();
        self.players.dedup();
    }

    fn prep_player_indices(&mut self) {
        let players = self.players.to_vec();

        for player in players {
            self.map_next_index(player);
        }
    }

    fn map_next_index(&mut self, player: Player) {
        let indices_total = self.trainer.get_next_generation().len();
        let mut indices_used: Vec<usize> = self.player_mapper.iter().map(|value| *value.1).collect();

        indices_used.sort();

        let index = {
            let mut index_opt = None;

            for test_index in 0..indices_total {
                if indices_used.binary_search(&test_index).is_err() && self.used_indices.binary_search(&test_index).is_err() {
                    index_opt = Some(test_index);
                    break;
                }
            }

            index_opt.unwrap_or_else(|| panic!("Tried to use indices after all were used"))
        };

        self.player_mapper.insert(player, index);
        // warn!("Mapped Player: {:?} to Index: {:?}", player, index);
        self.used_indices.push(index);
        self.used_indices.sort();
    }

    fn think(&mut self, player: Player, inputs: &mut Vec<W>) -> Box<Any + Send> {
        // let dot1 = dot(vec[0].1, vec[1].1);
        // let dot2 = dot(vec[1].1, vec[0].1);
        //
        // let mag = vec[0].1.distance(vec[1].1);

        // let inputs = vec!(vec!(mag as f64, dot1 as f64), vec!(mag as f64, dot2 as f64));

        let index = *self.player_mapper
            .get(&player)
            .unwrap_or_else(|| panic!("Player mapper get info.0 was none"));
        let network = self.trainer
            .get_next_generation()
            .get(&index)
            .unwrap_or_else(|| panic!("Next Gen Get Index is none"));

        // NeuralNetwork::sigmoid_inputs(inputs);

        let result: Vec<W> = network.fire(inputs);

        // warn!("Player: {:?}, Results: {:?}", player, result);

        let x: W = *result.get(0).unwrap_or_else(|| panic!("Panic"));
        let y: W = *result.get(1).unwrap_or_else(|| panic!("Panic"));

        let atan = y.atan2(x);

        Box::new(FromAi::Joy(atan.cos(), atan.sin(), player))

        // for info in inputs {
        //     let index = *self.player_mapper.get(&player).unwrap_or_else(|| panic!("Player mapper get info.0 was none"));
        //     let network = self.trainer.get_next_generation().get(&index).unwrap_or_else(|| panic!("Next Gen Get Index is none"));
        //
        //     let result = network.fire(&info);
        //
        //     output.push(
        //         match (result[0] * 4.0).abs().round() as i64 % 4 {
        //             0 => AiToControl::Right(result[1].min(1.0), player),
        //             1 => AiToControl::Left(result[1].min(1.0), player),
        //             2 => AiToControl::Up(result[1].min(1.0), player),
        //             3 => AiToControl::Down(result[1].min(1.0), player),
        //             _ => panic!("CRITICAL MATH ERROR"),
        //         }
        //     );
        // }

        // output
    }

    fn prep_reward(&mut self, reward: (Player, S)) {
        let index = *self.player_mapper
            .get(&reward.0)
            .unwrap_or_else(|| panic!("Player Mapper get Value.0 was None"));
        self.rewards.push((index, reward.1));
    }

    fn reward<F1: Fn() -> W, F2: Fn() -> W>(&mut self, reward: (Player, S), mutation_mult_picker: &Box<F1>, mutation_add_picker: &Box<F2>) {
        // warn!("Finished Game");
        self.prep_reward(reward);

        // warn!("Used Indices: {:?}, Next Gen Len: {:?}", self.used_indices.len(), self.trainer.get_next_generation().len());
        if self.used_indices.len() == self.trainer.get_next_generation().len() {
            self.train(mutation_mult_picker, mutation_add_picker);
        }
        self.player_mapper.clear();
        // self.print_unused_indices();
        self.prep_player_indices();
    }

    fn train<F1: Fn() -> W, F2: Fn() -> W>(&mut self, mutation_mult_picker: &Box<F1>, mutation_add_picker: &Box<F2>) {
        debug!("Training Next Generation");
        let mut rewards: HashMap<usize, S> = HashMap::new();

        for reward in self.rewards.drain(..) {
            let sum = {
                if rewards.contains_key(&reward.0) {
                    rewards.get(&reward.0).unwrap().clone()
                } else {
                    rewards.insert(reward.0, S::zero());
                    rewards.get(&reward.0).unwrap().clone()
                }
            };

            rewards.insert(reward.0, sum + reward.1);
        }
        for reward in &rewards {
            debug!("Index: {:?}, Fitness: {:?}", reward.0, reward.1);
        }
        self.trainer.train(rewards, mutation_mult_picker, mutation_add_picker);
        // warn!("Clearing Used Indices");
        self.used_indices.clear();
    }
}

pub struct AiSystem<ID: Eq + Ord, S: Debug + Ord + Clone + Decodable + Encodable + ToPrimitive + FromPrimitive + Num, W: Any + Send + Float + SampleRange + FromPrimitive + Decodable + Encodable, F1: Send + Fn() -> W, F2: Send + Fn() -> W> {
    control_channel_index: usize,
    feeder_channel_index: usize,
    channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>,
    // main_back_channel: BackChannel<Box<Into<ToAi<S, W>>>, EF1>,
    // feeder_back_channel: BackChannel<ET2, EF2>,
    // control_front_channel: FrontChannel<ET3, EF3>,
    brain_type: HashMap<Brain, BrainClump<S, W>>,
    brain_mapper: HashMap<Player, Brain>,
    mutation_mult_picker: Box<F1>,
    mutation_add_picker: Box<F2>,
}

impl<'a, 'b, ID, S, W, F1, F2> AiSystem<ID, S, W, F1, F2>
    where ID: Eq + Ord,
          S: Debug + Ord + Clone + Decodable + Encodable + ToPrimitive + FromPrimitive + Num,
          W: Any + Send + Float + SampleRange + FromPrimitive + Decodable + Encodable,
          F1: Send + Fn() -> W,
          F2: Send + Fn() -> W
{
    pub fn new(channels: Vec<DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>>,
               control_channel_id: ID,
               feeder_channel_id: ID,
               network_size_chase: Vec<S>,
               network_size_flee: Vec<S>,
               network_count: usize,
               min_weight: W,
               max_weight: W,
               min_bias: W,
               max_bias: W,
               mutation_mult_picker: Box<F1>,
               mutation_add_picker: Box<F2>)
               -> AiSystem<ID, S, W, F1, F2> {
        let input_size = S::from_u8(4).unwrap_or_else(|| panic!("S From u8 4 was none"));

        let mut brain_type: HashMap<Brain, BrainClump<S, W>> = HashMap::new();

        brain_type.insert(Brain::Chase,
                          BrainClump::<S, W>::load(Brain::Chase).unwrap_or_else(|| {
                              warn!("Load Brain Chase Failed");
                              BrainClump::new(network_count, input_size.clone(), network_size_chase, min_weight, max_weight, min_bias, max_bias)
                          }));

        brain_type.insert(Brain::Flee,
                          BrainClump::load(Brain::Flee).unwrap_or_else(|| {
                              warn!("Load Brain Flee Failed");
                              BrainClump::new(network_count, input_size.clone(), network_size_flee, min_weight, max_weight, min_bias, max_bias)
                          }));

        let mut system = AiSystem {
            channels: channels,
            control_channel_index: channels.binary_search_by_key(&&control_channel_id, |item| item.get_id()).unwrap_or_else(|err| panic!("{:?}", err)),
            feeder_channel_index: channels.binary_search_by_key(&&feeder_channel_id, |item| item.get_id()).unwrap_or_else(|err| panic!("{:?}", err)),
            // main_back_channel: main_back_channel,
            // feeder_back_channel: feeder_back_channel,
            // control_front_channel: control_front_channel,
            brain_type: brain_type,
            brain_mapper: HashMap::new(),
            mutation_mult_picker: mutation_mult_picker,
            mutation_add_picker: mutation_add_picker,
        };

        // system.map_player_to_brain(Player::One,
        //                            Brain::Chase);
        system.map_player_to_brain(Player::Two, Brain::Flee);

        system.prep_player_indices();

        system
    }

    fn prep_player_indices(&mut self) {
        for brain in self.brain_type.values_mut() {
            brain.prep_player_indices();
        }
    }

    fn map_player_to_brain(&mut self, player: Player, brain: Brain) {
        self.brain_mapper.insert(player, brain);
        self.brain_type
            .get_mut(&brain)
            .unwrap_or_else(|| panic!("Brain had no type"))
            .add_player(player);
    }

    fn process_event(&mut self, event: Box<ToAi<S, W>>) {
        let event = *event;
        match event {
            ToAi::WorldState(player, mut vec) => {
                let brain = match self.brain_mapper.get(&player) {
                    Some(brain) => brain,
                    None => return,
                };
                let thought = self.brain_type
                    .get_mut(brain)
                    .unwrap_or_else(|| panic!("Brain had no type"))
                    .think(player, &mut vec);
                self.get_mut_control_channel().unwrap_or_else(|| panic!("Control channel was none")).send(thought);
            }
            ToAi::Reward(vec) => {
                for reward in &vec {
                    let brain = match self.brain_mapper.get(&reward.0) {
                        Some(brain) => brain,
                        None => continue,
                    };
                    self.brain_type
                        .get_mut(brain)
                        .unwrap_or_else(|| panic!("Brain had no type"))
                        .prep_reward(reward.clone());
                }
            }
            ToAi::RewardAndEnd(vec) => {
                for reward in &vec {
                    let brain = match self.brain_mapper.get(&reward.0) {
                        Some(brain) => brain,
                        None => continue,
                    };
                    self.brain_type
                        .get_mut(brain)
                        .unwrap_or_else(|| panic!("Brain had no type"))
                        .reward(reward.clone(), &self.mutation_mult_picker, &self.mutation_add_picker);
                }
            }
        }
    }

    fn save(&self) {
        for brain in &self.brain_type {
            brain.1.save(*brain.0);
        }
    }

    fn get_mut_control_channel(&mut self) -> Option<&mut DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>> {
        let temp = self.control_channel_index;
        self.channels.get_mut(temp)
    }

    fn get_mut_feeder_channel(&mut self) -> Option<&mut DuoChannel<ID, Box<Any + Send>, Box<Any + Send>>> {
        let temp = self.feeder_channel_index;
        self.channels.get_mut(temp)
    }
}

impl<'a, 'b, ID, S, W, F1, F2> System<Delta> for AiSystem<ID, S, W, F1, F2>
    where ID: Eq + Send + Ord,
          S: Send + Ord + Clone + Decodable + Encodable + Debug + ToPrimitive + FromPrimitive + Num,
          W: Any + Send + Float + SampleRange + FromPrimitive + Decodable + Encodable,
          F1: Send + Fn() -> W,
          F2: Send + Fn() -> W
{
    fn run(&mut self, arg: RunArg, _delta_time: Delta) {

        while let Some(event) = self.get_mut_feeder_channel().unwrap_or_else(|| panic!("Feeder channel was none")).try_recv() {
            self.process_event(event);
        }

        if let Some(event) = self.get_mut_main_channel().unwrap_or_else(|| panic!("Main channel was none")).try_recv() {
            match event {
                ToAi::Save => {
                    self.save();
                    self.main_back_channel.send_from(FromAi::Saved);
                }
            }
        }

        arg.fetch(|_| ());
    }
}
