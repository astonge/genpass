use sha256::digest;
use chrono::{Utc};
use rand::Rng;

fn upper(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn make_words() -> (String, String) {
    let words = rand_word::new(2);
    let mut split = words.split(" ");

    return (
        upper(split.next().unwrap()).to_string(),
        upper(split.next().unwrap()).to_string()
    );
}

fn make_hash() -> String {
    return digest(Utc::now().to_string())[..10].to_string();
}

fn make_bang() -> String {
    let bangs = [
        "!!",
        "@@",
        "%%",
        "##",
        "??",
        "$$",
        "22",
        "88"
    ];

    return bangs[rand::thread_rng().gen_range(0..bangs.len())].to_string();
}

fn main()
{
    let (first, second) = make_words();
    
    println!("{}{}{}{}{}", 
        first,
        make_bang(),
        second,
        make_hash(),
        make_bang(),
    )
}