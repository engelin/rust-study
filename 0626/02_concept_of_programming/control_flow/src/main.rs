fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Error : since number is not bool, but integer
    //         the condition must be boolean type
    // if number {
    //    println!("number was three");
    // }
    
    if number != 0 {
        println!("number was something other than zero");
    }

    //////////////////////////////////////////////

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //////////////////////////////////////////////
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // Error : since "six" is not same data type from 5
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");


    //////////////////////////////////////////////
    // Loop
    //////////////////////////////////////////////
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // you can add the value you want returned after the break expression
            break counter * 2;
        }
    };

    println!("The result is {result}");


    //////////////////////////////////////////////
    // Loop labeling
    //////////////////////////////////////////////
    let mut count = 0;
    'counting_up: loop { // Loop label with '
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Break inner loop
            }
            if count == 2 {
                break 'counting_up; // Break outer loop labeled counting_up
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //////////////////////////////////////////////
    // While
    //////////////////////////////////////////////
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //////////////////////////////////////////////
    // For
    //////////////////////////////////////////////
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() { // in reverse order
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    for number in 1..4 { // in order
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
