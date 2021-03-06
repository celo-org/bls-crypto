use ark_serialize::CanonicalSerialize;
use bls_crypto::{hash_to_curve::try_and_increment::DIRECT_HASH_TO_G1, PrivateKey};

use clap::{App, Arg};
use ark_std::test_rng;
use std::{fs::File, io::Write};

fn main() {
    let matches = App::new("BLS Proof of Possession test vectors")
        .about("Generates many proof of posession for random keys")
        .arg(
            Arg::new("num")
                .short('n')
                .value_name("NUM")
                .help("Sets the number of test vectors")
                .required(true),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .value_name("OUT")
                .help("Sets the output file path")
                .required(true),
        )
        .get_matches();

    let num: i32 = matches.value_of("num").unwrap().parse().unwrap();
    let out = matches.value_of("out").unwrap();

    let try_and_increment = &*DIRECT_HASH_TO_G1;
    let rng = &mut test_rng();
    let mut file = File::create(out).unwrap();
    for _ in 0..num {
        let sk = PrivateKey::generate(rng);
        let pk = sk.to_public();
        let address_bytes = hex::decode("60515f8c59451e04ab4b22b3fc9a196b2ad354e6").unwrap();
        let mut pk_bytes = vec![];
        pk.serialize(&mut pk_bytes).unwrap();
        let pop = sk.sign_pop(&address_bytes, try_and_increment).unwrap();
        let mut pop_bytes = vec![];
        pop.serialize(&mut pop_bytes).unwrap();

        let mut sk_bytes = vec![];
        sk.serialize(&mut sk_bytes).unwrap();

        pk.verify_pop(&address_bytes, &pop, try_and_increment)
            .unwrap();

        file.write_all(
            format!(
                "{},{},{}\n",
                hex::encode(sk_bytes),
                hex::encode(pk_bytes),
                hex::encode(pop_bytes)
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
