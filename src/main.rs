extern crate clap;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate env_logger;
extern crate crypto_rs;
extern crate generator_rs;
extern crate serde_json;


use env_logger::Target;
use crypto_rs::el_gamal::encryption::{PublicKey, PrivateKey};
use generator_rs::generator::Generator;
use generator_rs::generator::Uciv;
use std::fs::File;
use std::io::Write;
use clap::{Arg, App, SubCommand};

/// Generates public and private election keys as well as
/// universal cast-as-intended verifiability information.
///
/// ```sh
/// USAGE:
///     generator_rs [keys | uciv] [-h | -v]
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// SUBCOMMANDS:
///     help    Prints this message or the help of the given subcommand(s)
///     keys    Generate a private/public keypair and writes to private_key.json and public_key.json
///     uciv    Generate UCIV information for voters
/// ```
fn main() {
    // init logger
    pretty_env_logger::formatted_builder().unwrap()
        //let's just set some random stuff.. for more see
        //https://docs.rs/env_logger/0.5.0-rc.1/env_logger/struct.Builder.html
        .target(Target::Stdout)
        .parse("generator_rs=trace")
        .init();

    let matches = App::new("generator_rs")
        .version("0.1.0")
        .author("Raphael Matile <raphael.matile@gmail.com>")
        .about("Generate cryptographic material for a vote")
        .usage("generator_rs [keys | uciv] [-h | -v]")
        .subcommand(
            SubCommand::with_name("keys")
                .about("Generate a private/public keypair and write to private_key.json and public_key.json")
        )
        .subcommand(
            SubCommand::with_name("uciv")
            .about("Generate UCIV information for voters and write to private_uciv.json and public_uciv.json")
            .arg(Arg::with_name("number_voters")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("The number of voters for which to generate UCIV information")
            )
            .arg(Arg::with_name("number_voting_options")
                .required(true)
                .takes_value(true)
                .index(2)
                .help("The number of available options a voter can choose from")
            )
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("uciv") => {
            let subcommand_matches = matches.subcommand_matches("uciv").unwrap();

            let number_voters: i64 = subcommand_matches.value_of("number_voters").unwrap().parse::<i64>().unwrap();
            let number_voting_options: i64 = subcommand_matches.value_of("number_voting_options").unwrap().parse::<i64>().unwrap();

            generate_uciv(number_voters, number_voting_options);
        },
        Some("keys") => {
            generate_keys();
        },
        Some(&_) | None => {
            // an unspecified or no command was used
            println!("{}", matches.usage())
        },
    }
}

fn generate_keys() {
    info!("Generating private and public key");
    let keys: (PrivateKey, PublicKey) = Generator::generate_keys();

    let mut public_key_file = File::create("./".to_owned() + "public_key.json").unwrap();
    let mut private_key_file = File::create("./".to_owned() + "private_key.json").unwrap();

    let public_key_data = serde_json::to_string_pretty(&keys.1).unwrap();
    public_key_file.write(public_key_data.as_bytes()).unwrap();

    let private_key_data = serde_json::to_string_pretty(&keys.0).unwrap();
    private_key_file.write(private_key_data.as_bytes()).unwrap();
    info!("Wrote public key to 'public_key.json' and private key to 'private_key.json");
}

fn generate_uciv(number_voters: i64, number_voting_options: i64) {
    info!("Generating UCIV for {:?} voters and {:?} voting options", number_voters, number_voting_options);
    info!("Using public key at path './public_key.json'");
    let public_key = PublicKey::new("public_key.json");

    let uciv: Uciv = Generator::generate_uciv(number_voters, number_voting_options, public_key);

    let mut public_uciv_file = File::create("./".to_owned() + "public_uciv.json").unwrap();
    let mut private_uciv_file = File::create("./".to_owned() + "private_uciv.json").unwrap();

    let public_uciv_data = serde_json::to_string_pretty(&uciv.public_uciv).unwrap();
    public_uciv_file.write(public_uciv_data.as_bytes()).unwrap();

    let private_uciv_data = serde_json::to_string_pretty(&uciv.private_uciv).unwrap();
    private_uciv_file.write(private_uciv_data.as_bytes()).unwrap();
    info!("Wrote public uciv to 'public_uciv.json' and private uciv to 'private_uciv.json");
}
