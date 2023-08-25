#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.height > other.width && self.width > other.height)
    }
    // method take over ownership of self and other (missing '&' to borrow)
    // after calling the method, it is not possible to use
    // self or the passed argument
    // Example:
    //     rect1.max(rect2);
    //     rect1.area(); <- not possible
    //     rect2.area(); <- not possible
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    // need write permission, you can only use it when the created Rectangle is mutable
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 45,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(25);

    println!("rect1 is {:?}", rect1);

    println!("The area of rect1 is {} square pixels.", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!(
        "Can rect1 hold rect3? {}",
        Rectangle::can_hold(&rect1, &rect3)
    );
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));

    println!("rect1.max(rect3): {:?}", rect1.max(rect3));
    // rect1 and rect3 can not be used anymore
}
