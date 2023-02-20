use sha256::digest;
use chrono::{Utc};
use rand::Rng; // 0.8.5

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
    let now = Utc::now();
    return digest(now.to_string())[..10].to_string();
}

fn make_bangbang() -> String {
    let things = [
        "!!",
        "@@",
        "%%",
        "##",
        "??",
        "$$",
        "22",
        "88"
    ];

    let num = rand::thread_rng().gen_range(0..things.len());

    return things[num].to_string();
}

fn main()
{
    let words = make_words();
    
    println!("{}{}{}{}{}", 
        words.0,
        make_bangbang(),
        words.1,
        make_hash(),
        make_bangbang(),
    )
}