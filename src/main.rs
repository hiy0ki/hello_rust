fn main() {
    println!("Hello, world!");

    println!("this is {}", add_one(3));

    for x in 1..11 {
        println!("{}", x);
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}
