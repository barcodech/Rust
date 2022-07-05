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
