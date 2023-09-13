fn main() {
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area1(width1, height1)
        );
    }
    {
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area2(rect1)
        );
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area3(&rect1)
        );
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("rect1 is {:#?}", rect1);
    }
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
