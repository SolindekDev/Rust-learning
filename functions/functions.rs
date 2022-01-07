pub fn function_string() -> String {
    return ("Hi it's a function that return string!").to_string();
}

pub fn function_int() -> i16 {
    return 128;
}

pub fn add(a: i16, b: i16) -> i16 {
    return a + b;
}

fn main() {
    println!("{}", function_string());
    println!("{}", function_int());
    println!("{}", add(1, 2));
}