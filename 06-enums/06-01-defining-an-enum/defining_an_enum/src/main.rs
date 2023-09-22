#![allow(unused)]
fn main() {
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        fn route(ip_type: IpAddrKind) {
            println!("{:?}", ip_type)
        }
        route(IpAddrKind::V4);
        route(IpAddrKind::V6);

        // 構造体と enum を使う
        // struct IpAddr {
        //     kind: IpAddrKind,
        //     address: String,
        // }
        // let home = IpAddr {
        //     kind: IpAddrKind::V4,
        //     address: String::from("127.0.0.1"),
        // };
        // let loopback = IpAddr {
        //     kind: IpAddrKind::V6,
        //     address: String::from("::1"),
        // };

        // enum には直接データを格納できる
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));

        // 各列挙子に紐付けるデータの型と量は、異なってもOK
        // これは構造体では不可
        enum IpAddr2 {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr2::V4(127, 0, 0, 1);
        let loopback = IpAddr2::V6(String::from("::1"));
    }
    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // enum もメソッドを定義できる
        impl Message {
            fn call(&self) {
                // method body would be defined here
                println!("{:?}", &self)
            }
        }

        let m1 = Message::Write(String::from("hello"));
        m1.call();
    }
    {
        // enum Option<T> {
        //     Some(T),
        //     None,
        // }
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
    }
    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        // let sum = x + y; // エラー
        // y が None でないことを示す必要がある
    }
}
