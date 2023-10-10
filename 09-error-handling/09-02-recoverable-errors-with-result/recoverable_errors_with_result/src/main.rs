#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    {
        // Resultで回復可能なエラー
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }
    {
        // match で Result を処理
        // let f = File::open("hello.txt"); // hello.text は存在しないので Err が発生
        // let f = match f {
        //     Ok(file) => file,
        //     Err(error) => {
        //         panic!("There was a problem opening the file: {:?}", error)
        //     }
        // };
    }
    {
        // エラーの種類によって条件分岐
        // let f = File::open("hello.txt");

        // let f = match f {
        //     Ok(file) => file,
        //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
        //         match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //             Err(e) => {
        //                 panic!("Tried to create file but there was a problem: {:?}", e)
        //             }
        //         }
        //     }
        //     Err(error) => {
        //         panic!("There was a problem opening the file: {:?}", error)
        //     }
        // };
    }
    {
        // 簡易にpanic!を実行する
        // let f = File::open("hello.txt").unwrap();

        // expectでpanic!時に任意のメッセージを出力できる
        // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    {
        // エラーを委譲する
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        let f = read_username_from_file();
    }
    {
        // エラー委譲のショートカット: ?演算子
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        let f = read_username_from_file();
    }
    {
        // ?演算子はReusltを返す関数内でか使用できない
        // let f = File::open("hello.txt")?; // <- これはエラーとなる
    }
}
