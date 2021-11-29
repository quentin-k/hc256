use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use clap::{App, Arg};

use hc256::Hc256;

fn main() {
    let matches = App::new("Hc256 Encryption Utility")
        .version("0.3.0")
        .author("Quentin K")
        .about("Encrypts a file using the hc-256 stream cipher")
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .value_name("KEY")
                .help("Base64 encoded 32 byte encryption key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key file")
                .short("K")
                .long("keyfile")
                .value_name("KEY FILE")
                .help("Encryption key file containing 32 raw bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("iv")
                .short("i")
                .long("iv")
                .value_name("IV")
                .help("Base64 encoded 32 byte initialization vector")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("iv file")
                .short("I")
                .long("ivfile")
                .value_name("IV FILE")
                .help("Initialization vector file containing raw bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File to encrypt")
                .takes_value(true)
                .required(true),
        )
        .arg(Arg::with_name("output file")
            .short("o")
            .long("output")
            .value_name("OUTPUT FILE")
            .help("Specify an output file to use instead of inplace encryption"))
        .get_matches();

    let key: [u8; 32] = {
        let v = match matches.value_of("key") {
            Some(v) => base64::decode(v).expect("You must supply a base64 encoded key"),
            None => match matches.value_of("key file") {
                Some(p) => {
                    let mut v = Vec::new();
                    let mut f = File::open(p).expect("You must enter a valid file path!");
                    f.read_to_end(&mut v).expect("Failed to read file");
                    v
                }
                None => {
                    eprintln!("You must supply either a base64 encoded key or key file");
                    exit(1);
                }
            },
        };

        v.try_into().expect("The key provided was not 32 bytes")
    };

    let iv: [u8; 32] = {
        let v = match matches.value_of("iv") {
            Some(v) => base64::decode(v).expect("You must supply a base64 encoded iv"),
            None => match matches.value_of("iv file") {
                Some(p) => {
                    let mut v = Vec::new();
                    let mut f = File::open(p).expect("You must enter a valid file path!");
                    f.read_to_end(&mut v).expect("Failed to read file");
                    v
                }
                None => {
                    eprintln!("You must supply either a base64 encoded initialization vector or initialization vector file");
                    exit(1);
                }
            },
        };

        v.try_into()
            .expect("The initialization vector provided was not 32 bytes")
    };

    let filename = matches.value_of("file").unwrap();
    let mut file = File::open(filename).expect("Please enter a valid file path");
    let mut content = Vec::new();
    file.read_to_end(&mut content).expect("Failed to read file");

    let mut cipher = Hc256::new(&key, &iv);
    cipher.apply_stream(&mut content);

    File::create(match matches.value_of("output file") {
        Some(p) => p,
        None => filename,
    })
        .unwrap()
        .write_all(&content)
        .expect("Failed to write content to file");
}
