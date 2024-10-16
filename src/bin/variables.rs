fn main() {
  //let is used to create a variable and is executed in the runtime 
  let value = 31;
  // println macro function is used to print values in the console. 
  println!("{:?}", value);

  // const is used to create a variable with constant value and executed in the compile time so no dynamic data is assigned to const variable
  // use UPPERCASE for declaring const variables
  const DATA: i32 = 30;
  println!("{:?}", DATA);

  //always use snake case to declare variable if there is multiple words for the varible
  let snake_case = 40;
  println!("{:?}", snake_case);
}