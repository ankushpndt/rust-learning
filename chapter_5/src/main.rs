fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{s1}' is {len}.");

    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
    // This code that attempts to create two mutable references to s will fail:

    let r1 = &mut s1;
    println!("{} ", r1);
    let r2 = &mut s1;
    println!("{}", r2);

    let reference_to_nothing = dangle();

    //slice type
    let str = String::from("Hello world");
    let test = find_first_word(&str);
    println!("For string {str} the result is {test}");
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("world");
    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn find_first_word(input: &str) -> &str {
    let s = input.as_bytes(); //convert string to an array of bytes as we have to go element by element
    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    &input[..]
}
