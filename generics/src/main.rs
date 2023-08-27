struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest_number = &number_list[0];

    for number in &number_list {
        if number > largest_number {
            largest_number = number;
        }
    }

    println!("The largest number is {}", largest_number);

    let number_list = vec![34, 50, 99, 25, 100, 65, 101, 199, 400, 7, 12];
    println!("The largest number is {}", largest_i32(&number_list));

    let char_list = vec!['g', 'j', 'k', 'z', 'h', 'y', 'q', 'j'];
    println!("The largest char is: {}", largest_char(&char_list));

    let integer_point = Point { x: 5, y: 25 };
    println!("p.x = {}", integer_point.x());
    let float_point = Point { x: 2.5, y: 0.4 };
    println!("p.x = {}", float_point.x());
    println!("distance = {}", float_point.distance_from_origin());
}

//fn largest<T>(list: &[T]) -> &T {}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
