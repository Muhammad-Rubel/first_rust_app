
fn main() {
    // let mut name = "John";
    // let age = 32;
    // let is_male = false;

    // if is_male {
    //     println!("{name} is Male, and he is {age} years old");
    // } else {
    //   println!("{name} is Female, and she is {age} years old")
    // }

    // array
    let arr: [i32; 4] = [1, 2, 3, 4]; // [type; size] array
    let slice: &[i32] = &arr[0..3]; // slice

    println!("Slice: {:?}", slice);
    println!("arr: {:?}", arr);
    println!("arr[0]: {:?}", arr[0]);
    println!("arr length: {:?}", arr.len());

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", tup.0);

}
