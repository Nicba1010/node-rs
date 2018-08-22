extern crate futures;
extern crate getopts;
extern crate node;

extern crate log;
extern crate simple_logger;

use node::p2p::node::Node;
use std::env;
use std::vec::Vec;



fn print_usage(program: &str, opts: &getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // init logger
    simple_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("b", "bootstrap", "the address on which to listen", "");
    opts.optopt("c", "connect", "the address to which to connect", "");
    opts.optflag("h", "", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    let mut node = Node::new();

    match matches.opt_str("b") {
        Some(addr) => node.listen(addr.parse().unwrap()),
        None => {}
    }

    match matches.opt_str("c") {
        Some(addr) => node.connect(addr.parse().unwrap()),
        None => {}
    }
}
