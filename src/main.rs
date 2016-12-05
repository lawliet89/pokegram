extern crate hyper;
extern crate rustc_serialize;
extern crate telegram_bot;

mod pokeapi;
mod rest;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::boxed::Box;
use rustc_serialize::json;

#[derive(RustcDecodable, Eq, PartialEq, Clone, Debug)]
struct Config {
    pokeapi_endpoint: String
}

fn main() {
    let config_path = env::args().nth(1).expect("Usage ./pokegram path_to_config.json (Use - to read from stdin)");
    let config_json = read_config(&config_path, io::stdin()).unwrap();
    let config = parse_config(&config_json).unwrap();

    let api = pokeapi::PokeApi::new(&config.pokeapi_endpoint);
    println!("{:?}", api.poke_type("ground").unwrap());
}

fn read_config<R>(path: &str, reader: R) -> Result<String, String>
        where R : std::io::Read {
    let mut file : Box<std::io::Read> = match path {
        "-" => {
            Box::new(reader)
        },
        path @ _ => {
            match File::open(path) {
                Ok(f) => Box::new(f),
                Err(err) => return Err(format!("Unable to read file because: {}", err))
            }
        }
    };

    let mut json = String::new();
    match file.read_to_string(&mut json) {
        Ok(_) => Ok(json),
        Err(err) => Err(format!("Unable to read config: {}", err))
    }
}

fn parse_config(json: &str) -> Result<Config, String> {
    match json::decode(&json) {
        Ok(x) => Ok(x),
        Err(err) => return Err(format!("Unable to decode JSON value {}", err))
    }
}
