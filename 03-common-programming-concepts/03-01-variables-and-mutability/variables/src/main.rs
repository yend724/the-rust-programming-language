fn main() {
    let mut x = 5;
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

    const MAX_POINTS: u32 = 100_000;

    let mut spaces = "   ";
    // spaces = spaces.len(); // mut の可変変数は型が違うとエラーになる
    let spaces = spaces.len(); // シャドーイングは型が違っても良い
}
