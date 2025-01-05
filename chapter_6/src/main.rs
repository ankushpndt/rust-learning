struct User {
    name: String,
    active: bool,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

struct AlwaysEqual; // unit type struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) ->bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let mut user1: User = User {
        name: String::from("Test"),
        active: true,
        email: String::from("test@gmail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("mutateemail@gmail.com");
    println!("{}", user1.email);

    let user_from_build_user = build_user(String::from("Test"), String::from("test@gmail.com"));
    println!("{}", user_from_build_user.name);

    //Creating Instances from Other Instances with Struct Update Syntax
    let user2 = User {
        name: String::from("User2"),
        ..user1 // cannot use user1 values stored on heap after this as ownership has been moved to user2
    };

    //tuple structs
    let red = Color(100, 0, 0);
    set_bg_color(red);
    let point = Point(1, 1, 1);
    move_point(point);
    //color can also be passed in move_point bcoz both point and color have same structure so it's difficult to differentiate between them
    //so to make them a different type tuple we use tuple structs
    //move_point(red) is not possible now

    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.calc_area()
    );
    println!("{:#?}", rect1);
    dbg!(&rect1); //dbg! takes ownership that's why using the reference of rect1
    let rect2 = Rectangle {
        width: 5,
        height: 5,
    };
    println!("Can rect1 hold rect2? {:?}", rect1.can_hold(&rect2));
    let sq = Rectangle::square(5);
    dbg!(&sq);
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        active: true,
        email,
        sign_in_count: 1,
    }
}

fn set_bg_color(color: Color) {
    println!(
        "Setting bg color R={}, G={}, B={}",
        color.0, color.1, color.2
    )
}

fn move_point(point: Point) {
    println!("Point is X={}, Y={}, Z={}", point.0, point.1, point.2)
}
