fn main() {
    // Integer Types
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    // signed: -(2^(n - 1)) to 2^(n - 1) - 1
    // unsigned 0 to 2^n - 1

    let u8: u8 = 255;
    println!("u8: u8 = 255: {u8}");

    // Number literals	Example
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    // Overflow
    // - Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // - Return the None value if there is overflow with the checked_* methods.
    // - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
    // - Saturate at the value’s minimum or maximum values with the saturating_* methods.

    let f64 = 2.0; // f64
    println!("let f64 = 2.0: {f64}");
    let f32: f32 = 3.0; // f32
    println!("let f32: f32 = 3.0: {f32}");

    // char 4 Bytes
    // U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");

    // #### Tuples ####
    // ################
    println!("#### Tuples ####");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{tup:?}");
    println!("{tup:#?}"); // pretty print

    let (_, y, _) = tup; // pattern matching to destructure
    let x = tup.0;
    let z = tup.2;
    println!("x={x}, y={y}, z={z}");

    let u: () = (); // empty tuple -> unit

    // #### Arrays ####
    // ################
    println!("#### Arrays ####");
    let a = [1, 2, 3, 4, 5];
    println!("{a:?}");

    //   [type; size]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{a:?}");

    //  [initial-value; size]
    let a = [3; 5]; // -> [3, 3, 3, 3, 3]
    println!("{a:?}");
    println!("0={}, 4={}", a[0], a[4]);
}
