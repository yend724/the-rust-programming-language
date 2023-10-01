#![allow(unused)]
fn main() {
    {
        // ベクタの生成
        let vec1: Vec<i32> = Vec::new();
        let vec2 = vec![1, 2, 3];
        println!("vec1 is {:?}", vec1);
        println!("vec1 is {:?}", vec2);
    }
    {
        // ベクタの更新
        let mut v = Vec::new(); // mut

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        println!("v is {:?}", v);
    }
    {
        let v = vec![1, 2, 3, 4, 5];

        // 添字もしくはgetでアクセスできる
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        match v.get(2) {
            //                      "3つ目の要素は{}です"
            Some(third) => println!("The third element is {}", third),
            //               "3つ目の要素はありません。"
            None => println!("There is no third element."),
        }

        // 存在しない要素へのアクセスはパニックとなる
        // let v = vec![1, 2, 3, 4, 5];
        // let does_not_exist = &v[100];
        // let does_not_exist = v.get(100);

        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6);　// error　可変と不変な参照を同時には存在させられない
        println!("The first element is: {}", first);
        v.push(6);
        println!("v is: {:?}", v);
    }
    {
        // ベクタ内の値を順に処理する

        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50; // 参照外し:https://doc.rust-jp.rs/book-ja/ch15-02-deref.html#%E5%8F%82%E7%85%A7%E5%A4%96%E3%81%97%E6%BC%94%E7%AE%97%E5%AD%90%E3%81%A7%E5%80%A4%E3%81%BE%E3%81%A7%E3%83%9D%E3%82%A4%E3%83%B3%E3%82%BF%E3%82%92%E8%BF%BD%E3%81%84%E3%81%8B%E3%81%91%E3%82%8B
        }
        println!("v is {:?}", v);
    }
    {
        // Enumを使って複数の型を保持
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row: Vec<SpreadsheetCell> = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        for i in &row {
            println!("{:?}", i);
        }
    }
}
