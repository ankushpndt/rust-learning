//Ownership is a set of rules that govern how a Rust program manages memory.
// Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.
//If any of the rules are violated, the program won’t compile.
//None of the features of ownership will slow down your program while it’s running.

//Rules of ownership
//A value always have a owner
//A value always have one owner at a time
//If owner goes out of scope then value is dropped
fn main() {
    let s: &str = "Hello"; //main is the owner of s
    let mut str = String::from("hello"); //creating string from string literal hello
                                         // str is stored on heap as we don't know it's size
    str.push_str(" world");
    println!("{str}");
    //String is mutable while string literal is not
    {
        let x = "Hey"; //X scrope starts
    } // X scope ends
      //drop function is called which automatically free up the memory

    //x value is copied to y
    let x = 2;
    let y = x;

    //string is made up of 3 things that are length, capacity and pointer
    // length is how much memory a variable is using
    // capacity is amount of memory String has received from the allocator
    // pointer points to the memory that holds the content of a string
    //here s1 is shallow copied to s2 ie length, capacity and pointer of s1 are copied not the actual content stored on heap
    // instead of shallow copy it is called move
    let s1 = String::from("hello");
    let s2 = s1; //rust makes s1 invalid so now only s2 points to the actual content ie hello

    // here s2 and s1 both are pointing to same location but when s1 and s2 tries to free up the memory which is known as double free error

    //we can use clone() to create deep copy but it will be an expensive operation

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let name = String::from("Ankush");
    takes_ownership(name);
    // println!("{name}");
    let s = give_ownership();
    println!("{s}");

    let ss = takes_and_gives_back(s);
    println!("{ss}");
    let (str, len) = calculate_len(ss);
    println!("String is {str} length is {len}");
}

fn takes_ownership(s: String) {
    println!("{s}")
}

fn give_ownership() -> String {
    let s = String::from("This is a string from give ownership function");
    s
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    println!("a_string");
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_len(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}
