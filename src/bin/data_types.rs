fn main() {
  //The main data types in rust are
  // 1) Scalar Types.
  // #integer: two types of integer are signed and unsigned
  //  &)signed: can store positive and negative numbers: i8,i16,i32,i64,i128,
  let positive_number : i32 = 34;
  let negative_number : i32 = -32;
  //  &)unsigned : can store only positive numbers : u8,u16,u32,u64,u128
  let positive_unsigned: u32 = 50;
  let negative_unsigned: u32 = -50; //to allowed throws compile error.
  // #Floating numbers: to store decimal point numbers: f32 and f64
  let floating_number: f32 = 50.67;
  let floating_number2: f64 = 2000.67;
  // #boolean: to store boolean values : bool
  let is_true: bool = true;
  let is_false: bool = false;
  
  // #character: used to store single letter and need to use single quotes: char
  let character: char = 'A';
  let not_allowed: char = "B"; // double quotes are not allowed throws compile time error
  
  // 2) Compound Types
  
  // #Tuples: A tuple is a fixed-size collection of values of different types and we can destructure and can access using index values:
  //  (dataTypes,..,..)
  let tuple: (i32,char,bool) = (450,'a',true);
  
  // Arrays: An array is a fixed-size collection of values of the same type and can access using index: [dataTypes,limit]
  let arr : [i32; 4] = [1,2,3,4];
  let arr : [i32; 4] = [1,2,3,4,5]; // to allowed only we can store 4 elements if the length is already declared

  // String: has two main string type: String and &str 

  // #)String:  A growable, heap-allocated string : String
  let name: String = String::from("sharoon");

  // #)&str: A string slice (borrowed reference to a string)
  let my_name: &str = "sharoon";
}