fn main() {
    println!("{} {}", "Hello", "world!"); // Hello world!
    println!("{0} {1}", "Hello", "Solindekkkk!"); // Hello Solindekkkk
    println!("{hi} {name}", hi="Hello", name="Solindek!"); // Hello Solindek!

    println!("{:?}", [1,2,3]); // [1, 2, 3]
    println!("{:#?}", [1,2,3]); 
    /*
    [
        1,
        2,
        3,
    ]
    */

    // Formats
    let format = format!("{} {}", "Hello", "World!");
    println!("{}", format); // Hello World!
}