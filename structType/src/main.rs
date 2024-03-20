#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size : u32) -> Rectangle {
        Rectangle { length : size, width : size }
    }
}

fn main() {
    let rect1 = Rectangle::square(3);

    println!("rect1 is {:#?}", rect1);
}