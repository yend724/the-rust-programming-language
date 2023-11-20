#![allow(unused)]
fn main() {
    // ライフタイムでダングリング参照を回避する
    {
        let r: i32 = 1;
        // スコープ外の値は参照できない
        // {
        //     let x = 5;
        //     r = &x;
        // }

        println!("r: {}", r);
    }
    // 借用チェッカー
    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }
    // 関数のジェネリックなライフタイム
    {
        // ERROR:ライフタイム引数が必要
        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }

        // 関数シグニチャにおけるライフタイム注釈
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
    {
        //  異なる具体的なライフタイムを持つString値への参照でlongest関数を使用する
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
        // ERROR: string2のライフタイム外
        // println!("The longest string is {}", result);
    }
    // ライフタイムの観点で思考する
    {
        // SUCCESS: yのライフタイムはxや戻り値のライフタイムとは何の関係もない
        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }

        let string1 = String::from("abcd");
        let string2 = "efghijklmnopqrstuvwxyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
    {
        // ERROR: resultはスコープを抜けると破棄される
        // 修正するためには参照ではなく所有された値を返す
        // fn longest<'a>(x: &str, y: &str) -> &'a str {
        //     // 本当に長い文字列
        //     let result = String::from("really long string");
        //     result.as_str()
        // }
    }
    // 構造体定義のライフタイム注釈
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    // ライフタイム省略
    {
        // ライフタイム注釈なしでコンパイルできる（入力ライフタイムが一つだけの時）
        // https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html#%E3%83%A9%E3%82%A4%E3%83%95%E3%82%BF%E3%82%A4%E3%83%A0%E7%9C%81%E7%95%A5
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        let my_string = String::from("hello world");

        let word = first_word(&my_string[..]);
        println!("{}", word);

        let my_string_literal = "hello world";

        let word = first_word(&my_string_literal[..]);
        println!("{}", word);

        let word = first_word(my_string_literal);
        println!("{}", word);
    }
    // メソッド定義におけるライフタイム注釈
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }

        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    // 静的ライフタイム
    {
        // この参照がプログラムの全期間生存できる事を意味する
        // バイナリに直接格納され、常に利用可能
        let s: &'static str = "I have a static lifetime.";
    }
    // ジェネリックな型引数、トレイト境界、ライフタイムを一度に
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result =
            longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
        println!("The longest string is {}", result);

        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
