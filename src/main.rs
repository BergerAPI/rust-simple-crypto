use clap::{App, Arg};
use std::process;

use rand::distributions::Alphanumeric;
use rand::Rng;

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

    let text: String = String::from(matches.value_of("text").unwrap());

    // NOTE: Key could be None
    let unwrapped_key = matches.value_of("key");

    let value = match matches.value_of("method").unwrap() {
        "encrypt" => encrypt(text, get_random_string(30)),
        "decrypt" => decrypt(text, String::from(unwrapped_key.unwrap())),
        _ => {
            println!("Thats not a valid method stupido");
            process::exit(0);
        }
    };

    println!("{}", value);
}

/**
 * Generating a random string
 */
fn get_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn find_index(string: &str, val: &str, index: i64) -> usize {
    string
        .find(val.chars().nth(index as usize).unwrap())
        .unwrap()
}

/**
 * Encrypting a String with the key
 */
fn encrypt(text: String, key: String) -> String {
    let mut encrypted_string = String::from("");

    // A-Z, a-z
    let alphabet: String = (b'A'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    // Iterating and appending the key n-value and the text n-value together
    for i in 0..(text.len() as i64) {
        let cur_result = find_index(&alphabet, &text, i) << find_index(&key, &key, i);

        encrypted_string += &format!("{}", cur_result);
    }

    // Applying some random numbers to confuse the hackers
    for _ in 0..((text.len() as f64 * (key.len() as f64).sqrt()).floor() as i64 - text.len() as i64)
    {
        encrypted_string += &format!("{}", rand::thread_rng().gen_range(0..10))
    }

    return encrypted_string + " - Encrypted with " + &key;
}

/**
 * Decrypting a String with the key
 */
fn decrypt(text: String, key: String) -> String {
    let real_lenght = (text.len() as f64 / (key.len() as f64).sqrt()).floor();

    return format!(
        "decrypting {}, {} with a size of {}",
        text, key, real_lenght
    );
}
