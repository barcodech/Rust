use std::collections::HashMap;
fn main(){

  let mut marks = HashMap::new();
  marks.insert("Rust", 85);
  marks.insert("Python", 90);
  marks.insert("HTML", 60);
  marks.insert("CSS", 70);

  println!("How many subjects? {}",marks.len());

  match marks.get("Python") {
    Some(value) => println!("You got {} for Python",value),
    None => println!("You didn't study Python")
  }

  marks.remove("CSS");

  for (subject,value) in &marks {
    println!("For {} you got {} points",subject,value);

  }

  println!("Did you study Java? {}", marks.contains_key("Java"))

}