use clap::{App, Arg};
use std::process;

use rand::distributions::Alphanumeric;
use rand::Rng;

fn main() {
    let random = rand::thread_rng();
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

    let text: String = String::from(matches.value_of("text").unwrap());

    // Key could be None
    let unwrapped_key = matches.value_of("key");
    let key;

    if unwrapped_key != None {
        key = String::from(unwrapped_key.unwrap());
    } else {
        // Generating a pseudo random 30 characters long string
        key = random
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
    }

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
fn encrypt(text: String, key: String) -> String {
    let new_lenght = (text.len() as f64 * (key.len() as f64).sqrt()).floor();

    return format!(
        "encrypting {}, {} with a new size of {}",
        text, key, new_lenght
    );
}

/**
 * Decrypting a String with the key
 */
fn decrypt(text: String, key: String) -> String {
    return format!("decrypting {}, {}", text, key);
}
