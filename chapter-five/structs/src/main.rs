// Basic struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Creating an instance of a struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{username}", username = user1.username);

    // A whole instance must be made mutable if required, individual fields cannot be made mutable
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // Dot notation to reference specific fields of an instance
    user2.username = String::from("adifferentusername123");
    println!("{username}", username = user2.username);

    // Factory functions common to create instances with predefined values
    // however, specifying many different arguments for fields can get verbose
    fn build_user_v1(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
    let user3 = build_user_v1(String::from("user3@example.com"), String::from("user3username"));
    println!("{username}", username = user3.username);

    // This factory function uses shorthand where if the
    // argument and field name are the same, they do not need to be repeated
    fn build_user_v2(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user4 = build_user_v2(String::from("user4@example.com"), String::from("user4username"));
    println!("{username}", username = user4.username);

    // Another instance can be referenced as part of struct update syntax to reduce repetition
    // while still allowing for some unique field values
    // let user5 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("user5@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // The above can be simplified further by using ..
    // this specifies that all other fields not explicitly set
    // should be copied from the other instance
    let user6 = User {
        email: String::from("user6@example.com"),
        ..user1
    };

    // important to note that if instance B makes use of fields from instance A
    // then instance C cannot then use the same fields from instance A as they have now "moved"

    // Tuple structs
    // Good for defining a type where the field names do not matter
    // Good for naming useful tuples that should have a different type from other tuples

    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-like structs
    // Have no fields or values
    // Useful for implementing a trait on some type when there is no data to include in the type itself (???)
}
