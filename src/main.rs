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

use std::collections::HashMap;
 
fn main() {
    let mut drink_hashmap = HashMap::new();
 
    drink_hashmap.insert(1, "cola");
    drink_hashmap.insert(1, "coffee");
    drink_hashmap.insert(1, "pepsi");
    drink_hashmap.insert(1, "est");
 
    println!("{:?}", drink_hashmap.get(&1));
}
 
use std::collections::HashMap;
 
fn main() {
    let drink_vec = vec!["est", "pepsi", "coffee", "coffee"];
 
    let mut drink_hashmap = HashMap::new();
 
    for drink in drink_vec {
      drink_hashmap.entry(drink).or_insert(true);
    }
    for (drink, true_or_false) in drink_hashmap {
        println!("Do we have {}? {}", drink, true_or_false);
    }
}
 
 
use std::collections::HashMap;
 
fn main() {
    let drink_vec = vec!["est", "pepsi", "coffee", "coffee"];
 
 
    let mut drink_hashmap = HashMap::new();
 
    for drink in drink_vec {
        let return_value = drink_hashmap.entry(drink).or_insert(0);
        *return_value +=1;
    }
 
    for (drink, number) in drink_hashmap {
        println!("{}, {}", drink, number);
    }
}
 
use std::collections::HashMap;
 
fn main() {
    let drink_vec = vec![
        ("cola", 20),
        ("coffee", 25),
        ("est", 10),
        ("coffee", 30),
        ("est", 15),
        ("cola", 25),
    ];
 
    let mut drink_hashmap = HashMap::new();
 
    for drink in drink_vec {
      drink_hashmap.entry(drink.0).or_insert(Vec::new()).push(drink.1);
    }
 
    for (drink, prices) in drink_hashmap {
        println!("{:?}: {:?}", drink, prices);
    }
}
 
 
 
 
 
