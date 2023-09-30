#![allow(unused)]
fn main() {
    {
        let some_u8_value = Some(3);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }

        if let Some(3) = some_u8_value {
            println!("three");
        }
    }
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // ... などなど
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        let mut count = 0;
        let coin = Coin::Quarter(UsState::Alabama);
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }

        println!("count is {}", count);

        // match を if let に変換
        let coin = Coin::Penny;
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }

        println!("count is {}", count);
    }
}
