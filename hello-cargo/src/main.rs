fn main() {
  // Declaring a string and displaying it
  let hello_message: &str = "Hello, world!";
  println!("{}", hello_message);

  // Shadowing a declared variable even with a new type
  let hello_message: i32 = 5;
  println!("Your number is {}", hello_message);

  // Increasing it's value
  let hello_message: i32 = hello_message + 27;
  println!("And now {}", hello_message);
}
