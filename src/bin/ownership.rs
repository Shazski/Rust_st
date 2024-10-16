//Ownership is a set of rules that govern how a Rust program manages memory
fn main() {
  //each value has an owner
  //there can be only one owner at a time
  //when the owner goes out of scope, the value will be dropped.
  let value :String = String::from("sharoon") ;
  ownership_fn(value); //if we pass the value to ownership_fn then ownership_fn becomes the owner of that value 
  // and the main dont have any access to that value so its shows the error.  
  // To resolve this issue we have to pass the reference of the value and also if we are passing value ie.i32 that will pass by reference by default
  ownership_fn(&value); // here we use the & symbol to make the reference. 
  println!("{:?}", value);
}

fn ownership_fn(val:String) {
  println!("valus is {:?}",val); // now the owner of value is owenership_fn if the scope ends then the memory is cleared.
}


fn ownership_fn(val:&str) { //here we have to make string slice for reference.
  println!("valus is {:?}",val); 
}


//also we can use the clone method to pass the reference
fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();  // Clone the string to create a separate copy

  println!("s1 = {}, s2 = {}", s1, s2);
}