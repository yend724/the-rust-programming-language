fn main() {
    {
        let mut s = String::from("hello world");
        let word = first_word1(&s);
        s.clear();
        println!("word is {}", word); // sがなくてもwordにはアクセスできる
    }
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        println!("hello = {}, world = {}", hello, world);

        let len = s.len();
        let hello_world = &s[0..len];
        println!("hello_world = {}", hello_world);

        let hello_world = &s[..];
        println!("hello_world = {}", hello_world);
    }
    {
        let mut s = String::from("hello world");
        let word = first_word2(&s);
        // s.clear(); // error! （エラー！）
        println!("the first word is: {}", word);
    }
    {
        // 文字列リテラルは&str型であり、スライスである
        let _s = "Hello, world!";
    }
    {
        let my_string = String::from("hello world");
        let word = first_word3(&my_string[..]);
        println!("the first word is: {}", word);

        let my_string_literal = "hello world";
        let word = first_word3(&my_string_literal[..]);
        println!("the first word is: {}", word);

        let word = first_word3(my_string_literal);
        println!("the first word is: {}", word);
    }
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];
        println!("the slice[0] is: {}", slice[0]);
    }
}

fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    // &strはスライス
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
