fn main() {
    another_function(5);

    print_labeled_measurements(10, 'h');
}

fn another_function(x :i32) {
    println!("The value of x is : {x}");
}


fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}