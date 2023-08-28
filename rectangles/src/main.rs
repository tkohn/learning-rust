#[derive(Debug, Copy, Clone)]
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
    // for compiling this method Copy and Clone trait is needed
    // with the trait the method 'fn max(self, other: Rectangle) -> Rectangle'
    // does not need Owner permission for self and other.
    // Important Note: This works because the struct does not use any Heap Memory,
    // adding a String property to Rectangle would result into a compilation error!
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
    // need write permission, you can only use it when the created Rectangle is mutable
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let mut rect1 = Rectangle {
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

    // mutable to test square1.set_to_max(rect2);
    let mut square1 = Rectangle::square(25);

    println!("rect1 is {:?}", rect1);

    println!("The area of rect1 is {} square pixels.", rect1.area());

    rect1.set_width(31);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!(
        "Can rect1 hold rect3? {}",
        Rectangle::can_hold(&rect1, &rect3)
    );
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));

    println!("rect1.max(rect3): {:?}", rect1.max(rect3));
    // rect1 and rect3 can not be used anymore

    println!("{:?}", square1);
    square1.set_to_max(rect2);
    println!("{:?}", square1);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        //sort_operations.push(value); // cannot move out in an FnMut closure
        r.width
    });

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
    println!("{:#?}", list);
}
