fn main() {
    // 덧셈
    let sum = 5 + 10;
    println!("sum: 5 + 10 = {sum}");

    // 뺄셈
    let difference = 95.5 - 4.3;
    println!("difference: 95.5 - 4.3 = {difference}");

    // 곱셈
    let product = 4 * 30;
    println!("product: 4 * 30 = {product}");

    // 나눗셈
    let quotient = 56.7 / 32.2;
    println!("quotient: 56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3; // 결괏값은 -1입니다
    println!("truncated: -5 / 3 = {truncated}");

    // 나머지 연산
    let remainder = 43 % 5;
    println!("remainder: 43 % 5 = {remainder}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    // let (x, y, z) = tup;
    let (_, y, _) = tup; // remove unused variable warning
    let temp = tup.1;
    println!("The value of y is : {y}, {temp}");
}
