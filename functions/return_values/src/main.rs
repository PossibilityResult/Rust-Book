fn main() {
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(x);

    println!("The value of x is now: {x}");
}

fn five() -> i32 {
    // There is no semicolon after the return value
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
