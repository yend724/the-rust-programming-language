fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // 借用
        println!("The length of '{}' is {}.", s1, len);
    }
    {
        let s = String::from("hello");
        change1(&s);
    }
    {
        // 可変な参照を渡すことで値を変更できる
        let mut s = String::from("hello");
        change2(&mut s);
        println!("The s is {}.", s);
    }
    {
        // 特定のスコープで、ある特定のデータに対しては、一つしか可変な参照を持てない
        let mut _s = String::from("hello");

        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2); // error
    }
    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            println!("{}", r1);
        } // r1はここでスコープを抜けるので、これは問題ない
        let r2 = &mut s;
        println!("{}", r2);
    }
    {
        let mut _s = String::from("hello");

        let _r1 = &_s;
        let _r2 = &_s;

        // let _r3 = &mut s; // error 不変な参照をして言える間は可変な参照は不可
        println!("{}, {}", _r1, _r2); //　不変な参照
    }
}

fn calculate_length(s: &String) -> usize {
    s.len() // sはStringへの参照
} // sは参照なのでドロップしない

fn change1(some_string: &String) {
    // some_string.push_str(", world"); // 借用した値を変更しようとするとエラーになる
    println!("The some_string is {}.", some_string);
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // <- sが`drop`するので&sの参照先がなくなりエラー
// }
