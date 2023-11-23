// Constants are always immutable and can be declared in the global scope, but
// they must be assigned from a constant expression and given an explicit type.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn shadowing() {
    let x = 5;
    
    // Variables can be shadowed even if they are immutable or in the same
    // block.
    let x = x + 1;
    
    {
        // Shadowing is undone at the end of blocks.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x is: {x}");
    
    // Shadowing can change a variable's type.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Numer of spaces: {spaces}");
}

fn main() {
    // Variables must be declared with 'mut' to be modified.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    shadowing();
    
    // Unused private constants would give a compiler warning.
    println!("Seconds in three hours: {THREE_HOURS_IN_SECONDS}");
}
