use crypto_rs::cai::uciv::{PreImageSet, ImageSet};
use crypto_rs::arithmetic::mod_int::ModInt;
use crypto_rs::arithmetic::mod_int::RandModInt;
use crypto_rs::el_gamal::encryption::PublicKey;
use std::vec::Vec;

#[derive(Clone)]
pub struct Uciv {
    pub public_uciv: Vec<ImageSet>,
    pub private_uciv: Vec<PreImageSet>
}

pub struct Generator {}

impl Generator {
    pub fn generate(number_voters: i64, number_voting_options: i64, public_key: PublicKey) -> Uciv {

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
                pre_image_set.clone()
            );

            private_uciv.push(pre_image_set);
            public_uciv.push(image_set);
        }

        Uciv {
            private_uciv,
            public_uciv
        }
    }
}