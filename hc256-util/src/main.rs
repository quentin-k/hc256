use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use clap::{App, Arg};

use hc256::Hc256;

fn main() {
    let matches = App::new("Hc256 Encryption Utility")
        .version("0.4.0")
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
            Arg::with_name("smart key")
                .short("s")
                .long("smartkey")
                .value_name("SMART KEY")
                .help("Attempts to base64 decode the key and if fails will try to open file, the order can be flipped with the <TOGGLE> flag")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("smart iv")
                .short("S")
                .long("smartiv")
                .value_name("SMART IV")
                .help("Attempts to base64 decode the initialization vector and if fails will try to open file, the order can be flipped with the <TOGGLE> flag")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("toggle")
                .short("t")
                .long("toggle")
                .value_name("TOGGLE")
                .help("Toggles the order of smart decoding resolution")
                .takes_value(false)
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

    let key: [u8; 32] = match (
        matches.value_of("key"),
        matches.value_of("key file"),
        matches.value_of("smart key"),
        matches.occurrences_of("toggle"),
    ) {
        (Some(v), _, _, _) => {
            match get_base64_val(v) {
                Some(v) => v,
                None => {
                    eprintln!("You must supply either a base64 encoded key or key file");
                    exit(1);
                }
            }
        }
        (_, Some(v), _, _) => {
            match get_file_val(v) {
                Some(v) => v,
                None => {
                    eprintln!("You must supply either a base64 encoded key or key file");
                    exit(1);
                }
            }
        }
        (_, _, Some(v), t) => {
            if t < 1 {
                match get_base64_val(v) {
                    Some(v) => v,
                    None => {
                        match get_file_val(v) {
                            Some(v) => v,
                            None => {
                                eprintln!("You must supply either a base64 encoded key or key file");
                                exit(1);
                            }
                        }
                    }
                }
            } else {
                match get_file_val(v) {
                    Some(v) => v,
                    None => {
                        match get_base64_val(v) {
                            Some(v) => v,
                            None => {
                                eprintln!("You must supply either a base64 encoded key or key file");
                                exit(1);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            eprintln!("You must supply either a base64 encoded key or key file");
            exit(1);
        }
    };

    let iv: [u8; 32] = match (
        matches.value_of("iv"),
        matches.value_of("iv file"),
        matches.value_of("smart iv"),
        matches.occurrences_of("toggle"),
    ) {
        (Some(v), _, _, _) => {
            match get_base64_val(v) {
                Some(v) => v,
                None => {
                    eprintln!("You must supply either a base64 encoded initialization vector or initialization vector file");
                    exit(1);
                }
            }
        }
        (_, Some(v), _, _) => {
            match get_file_val(v) {
                Some(v) => v,
                None => {
                    eprintln!("You must supply either a base64 encoded initialization vector or initialization vector file");
                    exit(1);
                }
            }
        }
        (_, _, Some(v), t) => {
            if t < 1 {
                match get_base64_val(v) {
                    Some(v) => v,
                    None => {
                        match get_file_val(v) {
                            Some(v) => v,
                            None => {
                                eprintln!("You must supply either a base64 encoded initialization vector or initialization vector file");
                                exit(1);
                            }
                        }
                    }
                }
            } else {
                match get_file_val(v) {
                    Some(v) => v,
                    None => {
                        match get_base64_val(v) {
                            Some(v) => v,
                            None => {
                                eprintln!("You must supply either a base64 encoded initialization vector or initialization vector file");
                                exit(1);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            eprintln!("You must supply either a base64 encoded initialization vector or initialization vector file");
            exit(1);
        }
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

fn get_base64_val(val: &str) -> Option<[u8; 32]> {
    let dv = base64::decode(val);

    let dv = match dv {
        Ok(v) => v,
        Err(_) => return None,
    };

    if dv.len() == 32 {
        Some(dv.try_into().unwrap())
    } else {
        None
    }
}

fn get_file_val(val: &str) -> Option<[u8; 32]> {
    let file = File::open(val);

    let mut file = match file {
        Ok(v) => v,
        Err(_) => return None,
    };

    let mut fc = Vec::new();

    if file.read_to_end(&mut fc).is_err() {
        None
    } else if fc.len() == 32 {
        Some(fc.try_into().unwrap())
    } else {
        None
    }
}
