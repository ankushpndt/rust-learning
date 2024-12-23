// const PI:f32  = 3.14;
fn main() {
    //difference between let and const
    //const is always immutable and type needs to be define
    //const can be declared as a global variable
    //difference between mut and shadowing
    //shadowing
    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("No of spaces {spaces}");

    //mut
    // let mut spaces = "  ";
    // spaces = spaces.len();
    // println!("No of spaces {spaces}");
    my_func(3);
}

fn my_func(x: i8) -> i8 {
    println!("The value is {x}");
    x
}
