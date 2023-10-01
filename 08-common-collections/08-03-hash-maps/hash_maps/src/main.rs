#![allow(unused)]
use std::collections::HashMap;
fn main() {
    {
        // ハッシュマップの作成
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        println!("scores is {:?}", scores);
    }
    {
        // タプルから生成
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("scores is {:?}", scores);
    }
    {
        // ハッシュマップと所有権
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // println!("field_name is {:?}", field_name);　<- error 所有権はムーブされる
    }
    {
        // ハッシュマップの値の取得

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        println!("score Blue is {:?}", score);

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }
    {
        // 値の更新
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);

        // key に値がなかったときのみ更新
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
