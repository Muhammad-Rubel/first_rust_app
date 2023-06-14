/*
    Primitive Types
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
    - u = unsigned (positive only) 
    - i = signed (positive and negative)
    - 8, 16, 32, 64, 128 = number of bits
    - as the number is bigger, the more memory/bits it takes
    - isize, usize = use the number of bits your system architecture is running (64 bits or 32 bits)
    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays
 */

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
  // Default is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 454545454545454545;

  // Find max size
  println!("Max i32: {}", std::i32::MAX); // std = standard library, std::i32::MAX is a constant
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Get boolean from expression
  let is_greater: bool = 10 < 5;

  let a1 = 'a';
  let face = '\u{1F600}';

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}