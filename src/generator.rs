use crypto_rs::arithmetic::mod_int::From;
use crypto_rs::arithmetic::mod_int::ModInt;
use crypto_rs::arithmetic::mod_int::RandModInt;
use crypto_rs::cai::uciv::{ImageSet, PreImageSet};
use crypto_rs::el_gamal::encryption::{PrivateKey, PublicKey};
use num::BigInt;
use num::One;
use num::Zero;
use std::str::FromStr;
use std::vec::Vec;

/// Universal Cast-as-Intended Verifiability (UCIV) Information.
/// Holds both, private and public UCIV.
#[derive(Clone)]
pub struct Uciv {
    /// The public UCIV used for verification of Cast-as-Intended (CaI) proofs.
    pub public_uciv: Vec<ImageSet>,
    /// The private UCIV used for generating of Cast-as-Intended (CaI) proofs.
    pub private_uciv: Vec<PreImageSet>,
}

/// Generate election cryptographic material
pub struct Generator {}

impl Generator {
    /// Generate a set of election keys:
    /// - an ElGamal PrivateKey
    /// - an ElGamal PublicKey
    pub fn generate_keys() -> (PrivateKey, PublicKey) {
        // TODO: no safe prime generator yet available in BigInt
        // See https://github.com/rust-num/num-bigint/issues/31
        let x = BigInt::from_str("896771263533775491364511200158444196377569745583").unwrap();
        //h := (g^x) mod p
        let h = BigInt::from_str("216354726151927782480677585315485875691753344522").unwrap();
        let g = BigInt::from_str("650614565471833138727952492078522919745801716191").unwrap();
        let p = BigInt::from_str("1449901879557492303016150949425292606294424240059").unwrap();
        let q = (p.clone() - BigInt::one()) / BigInt::from_str("2").unwrap();

        let priv_key = PrivateKey {
            p: ModInt::from_value_modulus(p.clone(), BigInt::zero()),
            q: ModInt::from_value_modulus(q.clone(), BigInt::zero()),
            g: ModInt::from_value_modulus(g.clone(), p.clone()),
            x: ModInt::from_value_modulus(x.clone(), p.clone()),
        };

        let pub_key = PublicKey {
            p: ModInt::from_value_modulus(p.clone(), BigInt::zero()),
            q: ModInt::from_value_modulus(q.clone(), BigInt::zero()),
            h: ModInt::from_value_modulus(h.clone(), p.clone()),
            g: ModInt::from_value_modulus(g.clone(), p.clone()),
        };

        (priv_key, pub_key)
    }

    /// Generate private and public Universal Cast-as-Intended verifiability (UCIV) information.
    ///
    /// - number_voters: The number of voters for which to generate private & public UCIV
    /// - number_voting_options: The number of available voting options
    /// - public_key: The election public key
    pub fn generate_uciv(number_voters: i64, number_voting_options: i64, public_key: PublicKey) -> Uciv {
        let mut public_uciv: Vec<ImageSet> = vec![];
        let mut private_uciv: Vec<PreImageSet> = vec![];

        // for each voter, create
        // - pre-images
        // - corresponding images
        for _i in 0..number_voters {
            let mut pre_images = vec![];

            for _j in 0..number_voting_options {
                pre_images.push(ModInt::gen_modint(public_key.q.clone()));
            }

            let pre_image_set = PreImageSet {
                pre_images
            };

            let image_set = ImageSet::new(
                public_key.g.clone(),
                pre_image_set.clone(),
            );

            private_uciv.push(pre_image_set);
            public_uciv.push(image_set);
        }

        Uciv {
            private_uciv,
            public_uciv,
        }
    }
}
