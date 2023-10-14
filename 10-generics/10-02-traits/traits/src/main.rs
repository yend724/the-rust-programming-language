#![allow(unused)]
use std::fmt::Display;
fn main() {
    {
        // traitの型の実装
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
            ),
        };
        println!("New article available! {}", article.summarize());

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());
    }
    {
        // traitのデフォルトの実装
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        // デフォルトを実装している
        impl Summary for NewsArticle {}

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
            ),
        };
        // New article available! (Read more...)
        println!("New article available! {}", article.summarize());
    }
    {
        // デフオルトのオーバーライド
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                // "（{}さんの文章をもっと読む）"
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }
    {
        // 引数としてのトレイト

        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        // impl Trait構文
        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }

        // トレイト境界構文 <- impl Trait構文と同等
        pub fn notify2<T: Summary>(item: &T) {
            // 速報！ {}
            println!("Breaking news! {}", item.summarize());
        }

        // 複数のトレイト境界を+構文で指定する（以下のように指定可能）
        // pub fn notify(item: &(impl Summary + Display)) {
        // pub fn notify<T: Summary + Display>(item: &T) {

        // where句を使ったより明確なトレイト境界
        // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        // where句で書き直し↓
        // fn some_function<T, U>(t: &T, u: &U) -> i32
        // 　　where T: Display + Clone,
        // 　　U: Clone + Debug
        // {
    }
    {
        // トレイトを実装している型を返す

        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }

        // `impl Trait`は一種類の型のみ返せる
        // 次はエラーとなる
        // fn returns_summarizable(switch: bool) -> impl Summary {
        //     if switch {
        //         NewsArticle {
        //             headline: String::from("Penguins win the Stanley Cup Championship!"),
        //             location: String::from("Pittsburgh, PA, USA"),
        //             author: String::from("Iceburgh"),
        //             content: String::from(
        //                 "The Pittsburgh Penguins once again are the best \
        //          hockey team in the NHL.",
        //             ),
        //         }
        //     } else {
        //         Tweet {
        //             username: String::from("horse_ebooks"),
        //             content: String::from("of course, as you probably already know, people"),
        //             reply: false,
        //             retweet: false,
        //         }
        //     }
        // }
        {
            // トレイト境界でlargest関数の実装
            fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
                let mut largest = list[0];

                for &item in list {
                    if item > largest {
                        largest = item;
                    }
                }

                largest
            }

            let number_list = vec![34, 50, 25, 100, 65];

            let result = largest(&number_list);
            println!("The largest number is {}", result);

            let char_list = vec!['y', 'm', 'a', 'q'];

            let result = largest(&char_list);
            println!("The largest char is {}", result);
        }
        {
            // トレイト境界を使用して、メソッド実装を条件分けする
            // use std::fmt::Display;
            struct Pair<T> {
                x: T,
                y: T,
            }

            // 常にnewメソッドを実装する
            impl<T> Pair<T> {
                fn new(x: T, y: T) -> Self {
                    Self { x, y }
                }
            }

            // PartialOrdトレイトとDisplayトレイトを実装している時のみ、 cmp_displayメソッドを実装する
            impl<T: Display + PartialOrd> Pair<T> {
                fn cmp_display(&self) {
                    if self.x >= self.y {
                        println!("The largest member is x = {}", self.x);
                    } else {
                        println!("The largest member is y = {}", self.y);
                    }
                }
            }
        }
        {
            // ブランケット実装
            // Displayトレイトを実装するあらゆる型にToStringトレイトを実装
            // impl<T: Display> ToString for T {
            //     // --snip--
            // }
        }
    }
}
