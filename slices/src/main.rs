fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    
    // We have the length of the first word, but no useful data to use it with.
    // Additionally, this no longer matches the actual state of s.
    println!("{}", word);
    
    // A slice gives us a reference to part of a string's data. Internally it is
    // a reference with a length that does not need to be aligned to the start
    // of a value. Slices should be considered as references.
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("'{}', '{}'.", hello, world);
    // Start and end values may be omitted from the range syntax.
    let hello = &s[..5];
    let world = &s[6..];
    println!("'{}', '{}'.", hello, world);
    // Omitting both values gives you a slice of the full value.
    let full = &s[..];
    println!("{}", full);
    
    let mut s = String::from("foo bar");
    let word = first_slice(&s);
    // The clear method takes a mutable reference to the string, meaning any
    // other references to it would be invalidated. This also invalidates the
    // word. The compiler would catch this, resulting in safer code.
    //s.clear();
    println!("the first word is: {}", word);
    s.clear();
}

// Get the length of the first word in a string.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

// Get a string slice's first word as a slice.
fn first_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}
