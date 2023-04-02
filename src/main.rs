use std::str;
use clap::{App, Arg, SubCommand};
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use std::fs::{File};
use std::io::{Read};
use rsa::pkcs1::{FromRsaPublicKey, ToRsaPublicKey};
use rsa::pkcs8::{FromPrivateKey, ToPrivateKey};

fn main() {
    let matches = App::new("RSA CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Encrypts and decrypts files using RSA")
        .subcommand(
            SubCommand::with_name("gen-keys")
                .about("Generate public/private key pair")
                .arg(
                    Arg::with_name("output")
                        .short('o')
                        .long("output")
                        .help("Output file name without extension")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("encrypt")
                .about("Encrypt content of a .txt file using public key")
                .arg(
                    Arg::with_name("input")
                        .short('i')
                        .long("input")
                        .help("Input .txt file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("key")
                        .short('k')
                        .long("key")
                        .help("Public key file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short('o')
                        .long("output")
                        .help("Output encrypted file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("Decrypt content of a .txt file using private key")
                .arg(
                    Arg::with_name("input")
                        .short('i')
                        .long("input")
                        .help("Input encrypted file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("key")
                        .short('k')
                        .long("key")
                        .help("Private key file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short('o')
                        .long("output")
                        .help("Output decrypted .txt file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("gen-keys") {
        let output = matches.value_of("output").unwrap();
        generate_keys(output);
    } else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let input = matches.value_of("input").unwrap();
        let key = matches.value_of("key").unwrap();
        let output = matches.value_of("output").unwrap();
        encrypt(input, key, output);
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let input = matches.value_of("input").unwrap();
        let key = matches.value_of("key").unwrap();
        let output = matches.value_of("output").unwrap();
        decrypt(input, key, output);
    }
}

fn generate_keys(output: &str) {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let priv_key_pem = priv_key.to_pkcs8_pem().expect("Failed to create private key PEM");
    let pub_key_pem = pub_key.to_pkcs1_pem().expect("Failed to create public key PEM");

    std::fs::write(format!("{}_private.pem", output), priv_key_pem.as_bytes()).expect("Unable to write private key");
    std::fs::write(format!("{}_public.pem", output), pub_key_pem).expect("Unable to write public key");

    println!("Private key saved to: {}_private.pem", output);
    println!("Public key saved to: {}_public.pem", output);
}

fn encrypt(input: &str, key: &str, output: &str) {
    let mut pub_key_pem = Vec::new();
    File::open(key).expect("Unable to open public key file")
        .read_to_end(&mut pub_key_pem)
        .expect("Unable to read public key file");
    let pub_key = str::from_utf8(&pub_key_pem).map(RsaPublicKey::from_pkcs1_pem).expect("Failed to parse public key").unwrap();
    let mut data = Vec::new();
    File::open(input).expect("Unable to open input file")
        .read_to_end(&mut data)
        .expect("Unable to read input file");

    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data).expect("Failed to encrypt");

    std::fs::write(output, &enc_data).expect("Unable to write output file");
    println!("Encrypted data saved to: {}", output);
}

fn decrypt(input: &str, key: &str, output: &str) {
    let mut priv_key_pem = Vec::new();
    File::open(key).expect("Unable to open private key file")
        .read_to_end(&mut priv_key_pem)
        .expect("Unable to read private key file");
    let priv_key = str::from_utf8(&priv_key_pem).map(RsaPrivateKey::from_pkcs8_pem).expect("Failed to parse private key").unwrap();

    let mut enc_data = Vec::new();
    File::open(input).expect("Unable to open input file")
        .read_to_end(&mut enc_data)
        .expect("Unable to read input file");

    let dec_data = priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data).expect("Failed to decrypt");

    std::fs::write(output, &dec_data).expect("Unable to write output file");
    println!("Decrypted data saved to: {}", output);
}

