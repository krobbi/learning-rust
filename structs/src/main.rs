struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/*
// Structs can't store references without a lifetime.
struct SliceUser {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
*/

// Tuple structs with no named fields.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit struct. Can implement traits but contains no data.
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(title: &str, user: &User) {
    println!("User - {}:", title);
    println!(" * Active:   {}", user.active);
    println!(" * Username: {}", user.username);
    println!(" * Email:    {}", user.email);
    println!(" * Sign ins: {}", user.sign_in_count);
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    
    user1.email = String::from("anotheremail@example.com");
    
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    // user1.username is invalidated because it was moved to user2.username and
    // has no Copy trait.
    //println!("User 1: {}", user1.username);
    
    user1.username = String::from("username1");
    
    print_user("user1", &user1);
    print_user("user2", &user2);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let _subject = AlwaysEqual;
    
    println!("R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("[{}, {}, {}]", origin.0, origin.1, origin.2);
    
    /*
    let user3 = SliceUser {
        active: true,
        username: "someusername456",
        email: "someone2@example.com",
        sign_in_count: 1,
    };*/
}
