pub fn run() {
  // print to console
  println!("Hello from the print.rs file");

  // Basic formatting
  println!("{} is from {}", "Brad", "Mass");

  // positional Arguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "Brad", activity = "football");

  // placeholder traits (for binary, hex, octal)
  println!("Binary: {:b} Hex: {:b} Octal: {:o}", 10, 10, 10);

  // placeholder for debug trait (to print array, touple etc)
  println!("{:?}", (12, true, "hello"));

  // basic math
  println!("10 + 10 = {}", 10 + 10);
}