const MAX_POINTS: u32 = 100_000;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum IpAddrkind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrkind,
    address: String,
}

fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x)
    }
    println!("The value of x is : {}", x);

    let guess: i32 = "42".parse().expect("Not a number!");

    let tup = (499, 'a', 1.0);
    let (x, y, z) = tup;
    println!("the value of y is: {}", tup.2);
    print_labeled_measurement(3, 'i');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is : {}", y);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username124"),
        active: true,
        sign_in_count: 1,
    };
    println!("The value of user1 is : {}", user1.username);
    user1.username = String::from("username12222");
    println!("The value of user1 is : {}", user1.username);
    struct Color(i32, i32, i32);
    let white = Color(255, 255, 255);

    let four = IpAddrkind::V4;
    let six = IpAddrkind::V6;

    let v = vec![100, 32, 55];
    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(0.33),
        SpreadsheetCell::Text(String::from("hello")),
    ];
}

fn print_labeled_measurement(value: i32, uint_label: char) {
    println!("The measurement is: {} {}", value, uint_label)
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     };
// }
