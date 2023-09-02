use oop::AveragedCollection;

fn main() {
    let mut list = AveragedCollection::new();
    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.add(5);
    println!("{}", list.average());
}
