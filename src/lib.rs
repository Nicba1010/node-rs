#![crate_type = "lib"]
#![crate_name = "node_rs"]

extern crate futures;
extern crate bytes;
extern crate rand;
extern crate uuid;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate sha1;
extern crate bincode;

extern crate num;
extern crate crypto_rs;

pub mod p2p;
pub mod protocol;
pub mod config;
pub mod chain;