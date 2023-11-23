use ferris_says::say; // Can import modules or specific items.
use std::io::{stdout, BufWriter}; // Import multiple items from module.

fn main() {
    // Type is inferred, but must be imported to be used explictly.
    let stdout = stdout();
    // Rust has a primitive 'str' type, and a more useful 'String' struct.
    let message = String::from("Hello fellow Rustaceans!");
    // Get a count from an iterator. No idea what <'_> is supposed to mean.
    let width = message.chars().count();
    
    // Mut means mutable (can be changed after first definition.) Not sure about
    // stdout.lock. Maybe something to do with making stdout blocking/flushing?
    let mut writer = BufWriter::new(stdout.lock());
    // The '&' passes a reference. Looks a lot like passing a pointer in C.
    // Assuming '&mut' means the referenced value can be mutated. Maybe
    // something can be declred as 'mut' but passed to another function that
    // can't mutate it. Borrowing probably comes into play around function
    // calls, not sure how that works though.
    // Unwrap is used on a 'Result'. A bit like a promise in JavaScript. Results
    // have an OK or an error value. Unwrap returns the OK value or panics with
    // the error value. Could be a convenient method of error handling for
    // simple command line tools.
    say(&message, width, &mut writer).unwrap();
}
