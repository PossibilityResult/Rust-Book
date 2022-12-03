fn main() {
    // Statements are instructions that perform some action and don't return a value
    let y = 6;

    // Expressions evaluate to a value
    let y = {
        let x = 3;
        // Expressions don't have semicolons (but statements do)
        x + 1
    };

    println!("The value of y is: {y}");
}
