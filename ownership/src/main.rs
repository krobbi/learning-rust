fn main() {
    let s = String::from("hello");
    
    // Passing a heap value to a function is like moving it to another variable,
    // the original variable becomes invalidated.
    // If a type implements the Copy trait, the value is copied by value instead
    // of moved and the ownership system can be ignored. String does not
    // implement Copy.
    takes_ownership(s);
    
    // This won't compile. The some_string parameter in the takes_ownership
    // function took ownership of the value of s, so s is invalidated.
    // Additionally, the value should have already been dropped when some_string
    // went out of scope at the end of the takes_ownership function. If we
    // wanted to use the value, one option could have been to make a clone of s
    // with the clone method, and either passed the clone to take_ownership, or
    // used the clone here.
    //println!("{}", s);
    
    let x = 5;
    
    makes_copy(x);
    
    // This is not a concern with integers, or any type that implements the Copy
    // trait. Passing x to makes_copy will copy the value of x into the
    // some_integer parameter. When the function is called, there will be two
    // copies of the data on the stack, one for the original x variable, and
    // another for the some_integer parameter. These variables may be mutated
    // without affecting the other variable.
    println!("{}", x);
    
    // The value in s1 was originally owned by some_string in gives_ownership,
    // but the returned value is now owned by s1.
    let s1 = gives_ownership();
    
    println!("{}", s1);
    
    // Unused return values should be automatically dropped.
    gives_ownership();
    
    let s2 = String::from("hello");
    
    // The s2 variable no longer owns a value after passing it to the a_string
    // parameter in the takes_and_gives_back function, but this parameter's
    // value is returned, and the s3 variable takes ownership of it. The result
    // of this function call is equivalent to 'let s3 = s2;'
    let s3 = takes_and_gives_back(s2);
    //println!("{}", s2);
    println!("{}", s3);
    
    // Assigning to a variable, passing a parameter, or returning a value
    // transfers ownership of the value and invalidates the original variable,
    // unless the value's type implements the Copy trait.
    
    // Giving back ownership of an original value and returning a new value from
    // a function could be done by returning a tuple, but references simplify
    // this.
    
    // The variable s3 has been passed as a reference here, so s3 does not lose
    // ownership of its value and can be used after the function call.
    // In Rust, assigning a variable or passing a parameter with a reference is
    // called borrowing.
    let len = calculate_length(&s3);
    
    // Unlike a function call, the println macro from earlier did not take
    // ownership of s3's value.
    println!("The length of '{}' is {}.", s3, len);
    
    // If we want to change a reference's value, it must be a mutable reference.
    // Mutable references are created with an &mut prefix. To create a mutable
    // reference, the owner must also be mutable. Here ownership of s3 is moved
    // to another variable named s3 which shadows the original but is mutable.
    let mut s3 = s3;
    
    change(&mut s3);
    
    println!("{}", s3);
    
    let r1 = &mut s3;
    println!("Reference 1: {}", r1);
    
    // r1 is invalidated because a new mutable reference to the value is
    // created.
    let r2 = &mut s3;
    println!("Reference 2: {}", r2);
    
    // Each owner can only have one mutable reference at a time. Creating a new
    // mutable reference invalidates any existing mutable reference. This
    // prevents data race bugs.
    //println!("Reference 1: {}", r1);
    
    // r2 is invalidated because a mutable and immutable reference can't point
    // to the same value at the same time.
    let r3 = &s3;
    // Mutable references cannot be used at the same time as immutable
    // references. At any time an owner's value can only be referenced in these
    // ways:
    //  * No references.
    //  * 1 or more immutable references and no mutable references.
    //  * Exactly 1 mutable reference and no immutable references.
    // References must always point to a valid value.
    println!("Reference 3: {}", r3);
    //println!("References 2 & 3: '{}', '{}'", r2, r3);
    
    // r3 is invalidated because it is an immutable reference, but the value it
    // is pointing to has been mutated.
    s3 = String::from("foo");
    println!("{}", s3);
    //println!("{}", r3);
    
    // This would be bad. See the dangle function below.
    //let reference_to_nothing = dangle();
    let owner_of_something = no_dangle();
    println!("{}", owner_of_something);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(mut some_integer: i32) {
    // This won't affect any variable that the integer was passed from, because
    // integers are copied by value and stored on the stack. some_integer will
    // be popped from the stack at the end of this function.
    some_integer += 1;
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    
    // Returning from a function gives ownership of the value to the variable
    // that receives it.
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // The a_string parameter owns its value while this function is being run,
    // but since a_string is also returned, ownership can be given back to a
    // variable in the caller function.
    a_string
}

fn calculate_length(s: &String) -> usize {
    // Here the string is passed as a reference using an & prefix. Unlike the
    // previous functions, s references the value passed to it, but does not
    // take ownership of it. If the value of s was passed from a variable, that
    // variable will still own the value after calling this function.
    // The s parameter goes out of scope at the end of this function, but its
    // value is not dropped because it doesn't own it.
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Here s owns a string value, and would drop its value at the end of the
// function, but a reference is returned. After returning, the reference no
// longer points to valid memory. Luckily, the compiler will catch this.
// Workarounds for this can be performed with the lifetime syntax.
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

// Instead we want to return ownership of the string. We don't want references
// to invalid memory.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
