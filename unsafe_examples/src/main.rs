use std::slice;

fn main() {
    println!("#### Dereferencing a Raw Pointer ####");
    let mut num = 5;

    let r1 = &num as *const i32; // *const -> immutable raw pointer
    let r2 = &mut num as *mut i32; // *mut -> mutable raw pointer

    // println!("r1 is: {}", *r1); // dereference of raw pointer is unsafe and requires unsafe function or block

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        // println!("r is: {}", *r);
        // thread 'main' panicked at 'misaligned pointer dereference: address must be a multiple of 0x4 but is 0x12345
        // [..]
        // thread caused non-unwinding panic. aborting.
    }

    println!("#### Calling an Unsafe Function or Method ####");
    // dangerous(); // call to unsafe function is unsafe and requires unsafe function or block
    unsafe {
        dangerous();
    }

    println!("#### Creating a Safe Abstraction over Unsafe Code ####");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(v.as_mut_slice(), 3);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("#### Using extern Functions to Call External Code ####");
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-5));
    }

    println!("#### Accessing or Modifying a Mutable Static Variable ####");
    println!("static HELLO_WORLD: {}", HELLO_WORLD);
    // println!("COUNTER: {}", COUNTER); // use of mutable static is unsafe and requires unsafe function or block
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    add_to_count(42);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

static HELLO_WORLD: &str = "Hello world!";
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    // COUNTER += inc; // use of mutable static is unsafe and requires unsafe function or block
    unsafe {
        COUNTER += inc;
    }
}
extern "C" {
    fn abs(input: i32) -> i32;
}

// Calling Rust Functions from Other Languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn dangerous() {}
