//normal functions
fn main() {
  println!("hello function"); 
  another_function(); 
  parameter_fn(30);
  multi_prameter(40,'T'); 
  let sum = returning_fn(10,20);
  println!("sum is {:?}", sum);
}

fn another_function() {
  println!("another function");
}

//parameter functions
fn parameter_fn(x:i32) {
  println!("{:?}", x);
}

fn multi_prameter(x:i32, letter: char) {
  println!("number is {:?} and letter is {:?}",x,letter);
}

//returning function

fn returning_fn(x:i32,y:i32) -> i32 {
  x + y // if we are not using semicol the it automatically return the last statement
  x + y; //this will not return the value
  return x + y; //this will return the value
}


