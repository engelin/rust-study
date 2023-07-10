fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// This definition can not be found by compiler.
// fn second_word(s: &String) -> (usize, usize) {
fn second_word(s: &String) -> &str { // Compiler ensure the reference's validation
    let bytes = s.as_bytes();

    let mut first_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_index == 0 {
                first_index = i;
            }
            else {
                return &s[first_index+1..i];
            }
        }
        else if i == s.len() - 1 && first_index != 0 {
            return &s[first_index+1..];
        }
    }

    if first_index != 0 {
        return &s[0..first_index];
    }

    return &s[..];
}

fn main() {
    let mut s = String::from("hello world zzang");

    let word = first_word(&s);
    println!("the first word is: {}", word);

    let word = second_word(&s);
    println!("the second word is: {}", word);

    s.clear();
    // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("the first word is: {}", word);
}
