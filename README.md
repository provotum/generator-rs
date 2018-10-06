generator-rs
================

Create cryptographic material used for an election:

* private / public key pair for the election
* universal cast-as-intended verifiability (UCIV) information for voters

This library is an implementation of [Universal Cast-as-Intended Verifiability (PDF)](https://fc16.ifca.ai/voting/papers/EGHM16.pdf)
by Alex Escala, Sandra Guasch, Javier Herranz, Paz Morillo.

**[WIP] This library is still work in progress and not audited in any way. Use at your own risk.** 

## Run

```
USAGE:
    generator_rs [keys | uciv] [-h | -v]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    keys    Generate a private/public keypair and write to private_key.json and public_key.json
    uciv    Generate UCIV information for voters and write to private_uciv.json and public_uciv.json
```

### Private / Public Keypair
To create an ElGamal private/public keypair, run
```
generator_rs keys
```
This will create two files in the project directory:
* `private_key.json`
* `public_key.json`

### Universal Cast-as-Intended Verifiability (UCIV) Information
:warning: :warning: :warning:

Before you'll able to create public and private UCIV information,
you have to generate a public key using the command described above.

:warning: :warning: :warning:

Then, execute the following command, but replace
* `<number of voters>` with the number of voters participating in the election/vote
* `<number of voting options>` with the amount of available voting options a voter can choose from
 
```
generator_rs uciv <number of voters> <number of voting options>
```

## Development

To build the library, run 
```
cargo build
```

To generate an updated set of the docs, run
```
cargo doc --no-deps
```

## Benchmarks

To run this test, you need to have nightly rust installed:
 ```
 rustup install nightly
 ```

Then run
 ```
 rustup run nightly cargo bench
 ```