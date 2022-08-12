use std::io;
 
fn main() {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();
 
    while input_string.trim() != "x\r\n" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {:?}", input_string);
    }
    println!("See you later!");
}
 

use std::io;

fn main() {
  println!("please input a message");
  let mut input = String:: new();
  io::stdin().read_line(&mut input).expect("failed to read input");
  println!("{}",input);
}


use std::env::args;
 
fn main() {
    let input = args();
 
    for entry in input {
        println!("You entered: {}", entry);
    }
}
 
use std::env::args;
 
fn main() {
    let input = args();
 
    input.skip(1).for_each(|item| {
        println!("You wrote {}, the capital letters is {}", item, item.to_uppercase());
    })
}
 
 
use std::env::args;
 
enum Letters {
    Capitalize,
    Lowercase,
    Nothing,
}
 
fn main() {
    let mut changes = Letters::Nothing;
    let input = args().collect::<Vec<_>>();
 
    match input[1].as_str() {
        "capital" => changes = Letters::Capitalize,
        "lowercase" => changes = Letters::Lowercase,
        _ => {}
    }
 
 
    for word in input.iter().skip(2) {
      match changes {
        Letters::Capitalize => println!("{}", word.to_uppercase()),
        Letters::Lowercase => println!("{}", word.to_lowercase()),
        _ => println!("{}", word)
      }
    }
   
}
 
 
 
 
 
