#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --omitted--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

fn main() {
    let current_coin = Coin::Penny;
    let ret = Coin::value_in_cents(current_coin);
    println!("{:?}", ret);

    let quarter_coin = Coin::Quarter(UsState::Alabama);
    let ret = Coin::value_in_cents(quarter_coin);
    println!("{:?}", ret);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("If let The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("coin count {:?}", count);

    let coin = Coin::Penny;
    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("coin count {:?}", count);
}
