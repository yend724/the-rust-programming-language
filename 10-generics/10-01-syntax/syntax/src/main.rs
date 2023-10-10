#![allow(unused)]

fn main() {
    {
        //ジェネリクス
        struct Point<T> {
            x: T,
            y: T,
        }

        // Point<i32>
        let integer = Point { x: 5, y: 10 };

        // Point<f64>
        let wont_work = Point { x: 1.0, y: 4.0 };

        // xとyの方が違うのでエラーとなる
        // let float = Point { x: 1, y: 4.0 };
    }
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }
    {
        // Option と Result はジェネリクス

        enum Option<T> {
            Some(T),
            None,
        }
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        // メソッド定義のジェネリクス
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        // Point<i32>なのでエラーとなる
        let p = Point { x: 5, y: 10 };
        // println!("p.distance_from_origin() = {}", p.distance_from_origin());

        // Point<f32>なのでOK
        let p = Point { x: 5.0, y: 10.0 };
        println!("p.distance_from_origin() = {}", p.distance_from_origin());
    }
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
    {
        // コンパイル時に型を決める
        let integer = Some(5);
        let float = Some(5.0);
    }
}
