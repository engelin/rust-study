fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    // Error: cannot borrow `s` as mutable more than once at a time
    // let r1 = &mut s;
    // let r2 = &mut s;

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);
    // // also cannot have a mutable reference while we have an immutable one to the same value

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // so, it becomes no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// This change function can't be compiled
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

/*
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
  --> src/main.rs:15:5
   |
15 |     some_string.push_str(", world");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference
   |
14 | fn change(some_string: &mut String) {
   |                        ~~~~~~~~~~~

For more information about this error, try `rustc --explain E0596`.
error: could not compile `references_and_borrowing` (bin "references_and_borrowing") due to previous error
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
