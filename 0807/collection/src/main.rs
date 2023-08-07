fn main() {
    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; // <-- immutable borrow occurs here

    v.push(6); // <-- mutable borrow occurs here

    // println!("The first element is: {first}"); // <-- error: immutable borrow later used here
    println!("The first element is: {}", v[0]); // <-- OK: v is mutable variable

    let v = vec![100, 32, 57];
    for i in v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}
