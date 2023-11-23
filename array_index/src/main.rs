use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    
    println!("Please enter an array index.");
    
    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");
    
    // Array indexing is always bounds checked at runtime. This can be bypassed
    // with 'get_unchecked' in an unsafe block.
    // A better option in most cases would be to handle the error properly.
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
