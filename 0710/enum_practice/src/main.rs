#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // dbg!(self);
        match self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("{x} {y}"),
            Message::Write(test) => println!("{:?}", test),
            Message::ChangeColor(test, ..) => println!("{:?}", test),
        }
    }
}

fn main() {
    let m = Message::Quit;
    m.call();

    let m = Message::Move{x: 32, y: 64};
    m.call();

    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::ChangeColor(1, 1, 0);
    m.call();
}
