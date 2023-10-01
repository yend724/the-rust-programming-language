#![allow(unused)]
fn main() {
    {
        // String
        let mut s = String::new();

        // &str
        let data = "initial contents";

        // String
        let s = data.to_string();
        let s = "initial contents".to_string();
        let s = String::from("initial contents");
    }
    {
        // 文字列の更新
        let mut s = String::from("foo");
        println!("s is {}", s);
        s.push_str("bar");
        println!("s is {}", s);

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2); // push_str は &str を引数に取る
        println!("s2 is {}", s2); // push_str が所有権を奪っていたらここでエラーとなる

        let mut s = String::from("lo");
        s.push('l');
        println!("s is {}", s);
    }
    {
        // 文字列の連結
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        // println!("s1 is {}", s1); // error s1はムーブされている
        println!("s2 is {}, s3 is {}", s2, s3);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s is {}", s);

        // format!
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s1 is {}, s2 is {}, s3 is {}", s1, s2, s3); // format!は所有権を奪わない
        println!("s is {}", s);
    }
    {
        let s1 = String::from("hello");
        // let h = s1[0]; <-error 文字列に添字でアクセスできない

        // 文字によってバイト数が違う
        let len1 = String::from("Hola").len();
        let len2 = String::from("Здравствуйте").len();
        println!("{}", len1);
        println!("{}", len2); // 12ではなく24

        // 文字列スライス
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("{}", s); // Зд

        // let s = &hello[0..1]; helloは2バイトずつなのでこれはパニックとなる
        // println!("s is {}", s);
    }
    {
        // chars()で一文字ずつアクセスできる
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
