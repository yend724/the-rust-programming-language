fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // 式と文
    let _y = 6; // 文は値を返さない

    // let x = (let y = 6);　// <- 値を返さないのでエラーとなる

    let y = {
        let x = 3;
        x + 1 // <- 式は値がリターンされるの結果が y に束縛される
    };
    println!("The value of y is: {}", y);

    println!("The value of five() is: {}", five());

    println!("The value of plus_one(5) is: {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
// 戻り値がある場合型が必要
fn five() -> i32 {
    5 // return がなければ最後の値を暗黙的に返す
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // <- セミコロンがあると文になるのでエラーとなる
}
