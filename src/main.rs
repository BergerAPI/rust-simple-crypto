extern crate clap;
use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("cryptography")
        .version("1.0")
        .author("bergerapi")
        .arg(
            Arg::with_name("method")
                .required(true)
                .short("m")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("text")
                .required(true)
                .short("t")
                .takes_value(true),
        )
        .arg(Arg::with_name("key").short("k").takes_value(true))
        .get_matches();

    let text = matches.value_of("text").unwrap();
    let key = matches.value_of("key").unwrap();

    let value = match matches.value_of("method").unwrap() {
        "encrypt" => encrypt(text, key),
        "decrypt" => decrypt(text, key),
        _ => {
            println!("Thats not a valid method stupido");
            process::exit(0);
        }
    };

    println!("{}", value);
}

/**
 * Encrypting a String with the key
 */
fn encrypt<'a>(text: &'a str, key: &'a str) -> String {
    return format!("encrypting {}, {}", text, key);
}

/**
 * Decrypting a String with the key
 */
fn decrypt<'a>(text: &'a str, key: &'a str) -> String {
    return format!("decrypting {}, {}", text, key);
}
