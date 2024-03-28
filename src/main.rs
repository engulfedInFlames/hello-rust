// Immutable vars can be assigend values only once.
// 


fn main(){
  /// Type aliasing
  type Age = u8;
  let peter_age: Age = 42;

  // Type Conversion
  let a = 10;
  let b = a as f64;

  ///// Compound Data Types: Hold multiple values /////
  // &str and String
  let fixed_str = "Fixed length string";
  let mut flexible_str = String::from("This string will grow and shrink as needed.");
  flexible_str.push('s');

  // Array
  let mut arr1 = [0;5]; // Declare default value
  let el1 = arr1[3];

  println!("{:?}", arr1);

  // Vector
  let vec1 : Vec<i32> = vec![1, 2, 3, 4, 5]; // NO size info. Vector type can grow and shrink
  let el2 = vec1[3];

  // Tuple
  let person_info = ("Salary", 30000, "Age", 27);
  let salary_val  = person_info.1;
  let (salary, salary_val, age, age_val ) = person_info;

  let unit = (); // Unit type. Similar to void in C; is zero sized and do NOT consume any memory

  // Fn and Code Block

  // Condition

  //
}