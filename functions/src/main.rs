fn main() {
    print_labeled_measurement(plus_one(five()), 'h');
}

// Expressions in statements without a semicolon become implicit returns.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}
