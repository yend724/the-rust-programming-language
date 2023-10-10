#![allow(unused)]
fn main() {
    {
        // panic!("crash and burn"); <-クラッシュ
    }
    {
        let v = vec![1, 2, 3];
        // v[99]; //<- クラッシュ
    }
}
