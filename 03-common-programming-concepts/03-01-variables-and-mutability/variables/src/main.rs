fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    const _MAX_POINTS: u32 = 100_000; // 定数はconst

    // let mut spaces = "   ";
    // spaces = spaces.len(); // mut の可変変数は型が違うとエラーになる
    let spaces = "   ";
    let spaces = spaces.len(); // シャドーイングは型が違っても良い
    println!("The value of spaces is: {}", spaces);
}
