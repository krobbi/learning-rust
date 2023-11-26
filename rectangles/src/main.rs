struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods without self references can be used as constructors/statics.
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Fields can share names with methods.
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect3));
    
    let sq = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", sq.area());
}
