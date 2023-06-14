// variables hold primitive data or references to data
// variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Brad";
  let mut age = 37;

  println!("My name is {} and I am {}", name, age);
  age = 38;
  println!("My name is {} and I am {}", name, age);

//  define constant ( use Capital letters for naming a constans and contants must be explecitely typed )
const ID: i32 = 001;
println!("ID: {}", ID);

// Assain multiple vars
let (my_name, my_age) = ("Brad", 37);
println!("{} is {}", my_name, my_age);

}