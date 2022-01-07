fn lang(lang: &str) -> i32 {
    match lang {
        "C" => return 10,
        "Go" => return 8,
        "Java" => return -100000,
        "Rust" => return 10,
        "JavaScript" => return 5,
        "PHP" => return -10,
        _ => return 0
    }
}

fn main() {
    println!("{}", lang("Java"));
}