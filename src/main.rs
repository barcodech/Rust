use std::fmt;
 
struct Dog {
    name: String,
    age: u8,
}
 
impl fmt::Display for Dog {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} is {} years old", self.name, self.age)
  }
}
 
fn main() {
    let dog1 = Dog {
        name: "Jackky".to_string(),
        age: 2,
    };
 
    println!("{}", dog1);
    println!("Jackky's String is {} letters long.", dog1.to_string().chars().count());
}
use std::fmt::Display;
 
fn print_vec<T: Display>(input: &Vec<T>) {
    for item in input {
        print!("{} ", item);
    }
    println!();
}
 
fn main() {
 
    let array_vec = Vec::from([1, 2, 3]);
    print_vec(&array_vec);
 
    let str_vec = Vec::from("string slide");
    print_vec(&str_vec);
 
    let string_vec = Vec::from("string".to_string());
    print_vec(&string_vec);
}
