fn main() {
    let number = 7;
    
    // Condition does not need parentheses, but does need braces.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    let number = 3;
    
    // Conditions must be explicit booleans.
    //if number {
    if number != 0 {
        println!("number was something other than zero");
    }
    
    let number = 6;
    
    // Else if chains are supported.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // If is an expression, so it can perform the role of a ternary operator.
    // Branch types must match, even if one branch is unreachable.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is {number}");
    
    // This could be interrupted with Ctrl+C.
    /*loop {
        println!("again!");
    }*/
    
    // We can break with a value to return from a loop.
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is: {result}");
    
    let mut count = 0;
    
    // Break and continue can be used with 'labels:
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            
            if remaining == 9 {
                break;
            }
            
            if count == 2 {
                break 'counting_up;
            }
            
            remaining -= 1;
        }
        
        count += 1;
    }
    
    // While loops are available.
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");
        
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
    
    // For loops are available for collections.
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
    
    // Ranges can be represented with a literal expression.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    
    println!("LIFTOFF!!!");
}
