struct Counter {}
impl Iterator for Counter {
    type Item = u32; // Associated Types

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

// trait Add<Rhs=Self>
// Rhs -> right hand side
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
