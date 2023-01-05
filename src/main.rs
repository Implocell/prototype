pub mod templates;

use core::panic;

pub trait Type {
    fn write_files(&self, prototype_name: &str) -> Result<(), String>;
}

enum AcceptedTypes {
    React,
    Node,
}

pub struct Prototype {
    pub name: String,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct SpecificPrototypeConfig {
    rootPath: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PrototypeConfig {
    react: Option<SpecificPrototypeConfig>,
    node: Option<SpecificPrototypeConfig>,
}

struct Node {
    config: SpecificPrototypeConfig,
}
impl Type for Node {
    fn write_files(&self, prototype_name: &str) -> Result<(), String> {
        let cfg = &self.config;
        let actual_path = cfg.rootPath.to_owned() + "/" + &prototype_name + ".ts";
        match std::fs::write(actual_path, templates::node::render_node(prototype_name)){
            Ok(()) => println!("files successfully written"),
            Err(err) => panic!("{}", err)
        }
        Ok(())
    }
}

struct React {
    config: SpecificPrototypeConfig,
}

impl Type for React {
    fn write_files(&self, prototype_name: &str) -> Result<(), String> {
        let cfg = &self.config;
        let actual_path = cfg.rootPath.to_owned() + "/" + &prototype_name + ".tsx";
        match std::fs::write(actual_path, templates::react::render_react(prototype_name)){
            Ok(()) => println!("files successfully written"),
            Err(err) => panic!("{}", err)
        }
        Ok(())
    }
}

fn get_type_from_args(proto_type: &str) -> Result<AcceptedTypes, String> {
    let t = match proto_type.to_lowercase().trim() {
        "react" => AcceptedTypes::React,
        "node" => AcceptedTypes::Node,
        _ => return Err(String::from("the provided type is not valid")),
    };
    Ok(t)
}

fn initialize_config(t: AcceptedTypes, config: PrototypeConfig) -> Box<dyn Type + 'static>{
    match t {
        AcceptedTypes::React => Box::new(React{config: config.react.unwrap()}),
        AcceptedTypes::Node => Box::new(Node{config: config.node.unwrap()}),
    }
}

fn main() {
    let config = get_config_file();

    let prototype_name = std::env::args().nth(1).expect("missing prototype name");
    let prototype_type = std::env::args().nth(2).expect("missing type");

    let verified_prototype_name = match verify_prototype_name(prototype_name) {
        Ok(name) => name,
        Err(err) => panic!("{}", err),
    };

    let config_types = get_config_types(prototype_type, config);

    for c in config_types {
        c.write_files(&verified_prototype_name).err();
    }

    
}

fn get_config_types(prototype_type: String, config: PrototypeConfig) -> Vec<Box<dyn Type + 'static>> {
    let ts = prototype_type.split(',');
    let mut result:Vec<Box<dyn Type + 'static>>  = vec![];
    for t in ts {
        match get_type_from_args(t) {
            Ok(t) => result.push(initialize_config(t, config.clone())),
            Err(err) => panic!("{}", err),
        };
    }

    result
}

fn verify_prototype_name(p: String) -> Result<String, String> {
    if p.split(' ').count() > 1 {
        return Err(String::from("only one word allowed as prototype name"));
    }
    Ok(p.trim().to_string())
}

fn get_config_file() -> PrototypeConfig {
    let config = match std::fs::read_to_string("./prototype_config.json") {
        Ok(c) => c,
        Err(e) => panic!(
            "can't find prototype_config.json in current scope, err: {}",
            e
        ),
    };
    serde_json::from_str::<PrototypeConfig>(&config).expect("config file was not well-formatted")
}
