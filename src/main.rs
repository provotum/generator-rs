extern crate getopts;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate env_logger;
extern crate crypto_rs;
extern crate generator_rs;
extern crate serde_json;

use env_logger::Target;
use std::env;
use std::vec::Vec;
use crypto_rs::el_gamal::encryption::PublicKey;
use generator_rs::generator::Generator;
use generator_rs::generator::Uciv;
use std::fs::File;
use std::io::Write;

fn main() {
    // init logger
    pretty_env_logger::formatted_builder().unwrap()
        //let's just set some random stuff.. for more see
        //https://docs.rs/env_logger/0.5.0-rc.1/env_logger/struct.Builder.html
        .target(Target::Stdout)
        .parse("generator=trace")
        .init();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help menu");

    if args.len() < 3 {
        print_usage(&program, &opts);
        return;
    }

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    let number_voters = args[1].parse::<i64>().unwrap();
    let number_voting_options = args[2].parse::<i64>().unwrap();

    info!("Generating UCIV for {:?} voters and {:?} voting options", number_voters, number_voting_options);
    info!("Using public key at path './public_key.json'");
    let public_key = PublicKey::new("public_key.json");

    let uciv: Uciv = Generator::generate(number_voters, number_voting_options, public_key);

    let mut public_uciv_file = File::create("./".to_owned() + "public_uciv.json").unwrap();
    let mut private_uciv_file = File::create("./".to_owned() + "private_uciv.json").unwrap();

    let public_uciv_data = serde_json::to_string_pretty(&uciv.public_uciv).unwrap();
    public_uciv_file.write(public_uciv_data.as_bytes()).unwrap();

    let private_uciv_data = serde_json::to_string_pretty(&uciv.private_uciv).unwrap();
    private_uciv_file.write(private_uciv_data.as_bytes()).unwrap();
}

fn print_usage(program: &str, opts: &getopts::Options) {
    let brief = format!("Usage: {} no_voters no_voting_options [options]", program);
    print!("{}", opts.usage(&brief));
}
