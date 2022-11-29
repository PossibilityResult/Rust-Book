use std::io;

// Fixed size chunk of memory w/ same types stored in stacked
fn main() {
    // Elements of arrays have the same type
    let a = [1, 2, 3, 4, 5];

    // Useful when you know the number of elements
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    // [type; size]
    let a : [i32; 5] = [1, 2, 3, 4, 5];

    // Same value initializations
    let b = [3; 5];

    // Accessing Elements
    let first = a[0];
    let second = a[1];

    println!("Please enter an array index!");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a nubmer");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
