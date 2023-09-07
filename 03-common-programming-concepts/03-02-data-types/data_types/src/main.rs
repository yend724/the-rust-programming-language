use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // 複数の型が推論される時は明示的に型をつける必要がある
    println!("The value of guess is {}", guess);

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // addition
    // 足し算
    let sum = 5 + 10;
    println!("The value of sub is {}", sum);

    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;
    println!("The value of difference is {}", difference);

    // multiplication
    // 掛け算
    let product: i32 = 4 * 30;
    println!("The value of product is {}", product);

    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is {}", quotient);
    let floored = 2 / 3;
    println!("The value of floored is {}", floored); // Results in 0

    // 余り
    let remainder = 43 % 5;
    println!("The value of remainder is {}", remainder);

    let _true = true;
    let _false: bool = false;

    // char 型はシングルクォーテーション
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 255);
    println!("The value of tup is ({}, {}, {})", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("The value of tup is ({}, {}, {})", x, y, z);

    // 配列型は全ての要素が同じ型である必要がある
    let arr = [1, 2, 3, 4, 5];
    println!(
        "The value of arr is [{}, {}, {}, {}, {}]",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr = [3; 5]; // [3, 3, 3, 3, 3];

    // 無効な配列アクセスした場合
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
