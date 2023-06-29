fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // Error implicitly returns `()` as its body has no tail or `return` expression
              // 6 |     x + 1; // Error
              //   |          - help: remove this semicolon to return this value
    x + 1
}


fn main() {
    let _y = 6; // Statement : doesn't return the result
    // let x = (let y = 6); // Compile error since there is no result to bind to x : error: expected expression, found `let` statement

    let y = {
        let x = 3;
        x + 1
    };
    // {
    //     let x = 3;
    //     x + 1 ////// Do not use ; if use ";",
    //                  this will be a statement and wouldn't return the result
    // }; // Expression

    println!("The value of y is : {y}");

    let z = five();
    println!("The value of z is : {z}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}
