fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // これはエラーとなる
    // if number { // 条件式はbool型である必要がある
    //     println!("condition was true");
    // }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

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

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);

    // if式の結果を束縛する場合、返す型は同じである必要がある
    // let number = if condition { 5 } else { "six" }; // 条件によって型が異なるのでエラーとなる

    // loop { // 無限ループ
    //     println!("again!");
    // }

    let mut count = 0;
    // 'counting_upというラベルを設定
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // 内側のloopを終了
            }
            if count == 2 {
                break 'counting_up; // 外側のloopを終了
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    // 発射！
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let range = 1..4;
    println!("Range is {:?}", range)
}
