fn main() {
    println!("Hello, world!");

    println!("this is {}", add_one(3));
}


fn add_one(x: i32) -> i32 {
    x + 1
}
