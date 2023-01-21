#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.length * rectangle.width
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn equal(&self, another_rect: &Rectangle) -> bool {
        self.length == another_rect.length && self.width == another_rect.width
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.length > another_rect.length && self.width && another_rect.width
    }
}

fn main() {
    let rect = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 30, ..rect };

    println!("rect equal rect2?: {}", rect.equal(&rect2));

    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("rect is Rectangle {:#?}", rect);
}