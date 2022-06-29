fn return_number<MyType>(number: MyType) -> MyType {
    number
}
fn main() {
    let number1 = return_number(5);
    let number2 = return_number(0.5);
    println!("{} and {}",number1,number2);
}
use std::fmt::Display;
fn two_things<T: Display, U: Display>(statement: T, number: U) {
    println!("{} and {}", statement, number);
}
fn main() {
    two_things("Hello there!", 10);
}
use std::fmt::Debug;
#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}
fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}
fn main() {
    let dog = Animal {
        name: "Jack".to_string(),
        age: 1,
    };
 
    let number = 10;
 
    print_item(dog);
    print_item(number);
}
use std::fmt::Display;
use std::cmp::PartialOrd;
 
fn compare<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{} {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}
 
fn main() {
    compare("compare", 9, 8);
}
fn compare<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,

enum Option<T> {
  Some(T),
  None,
} 

fn option1(x: i32) -> Option<String> {
  match x > 2 {
      true => Some(String::from("result")),
      false => None,
  }
}
enum Result<T, E> {
  Ok(T),
  Err(E),
}

fn error1(dir: &str) {
 
  println!("\n\n");
 
  let mut list_cmd = std::process::Command::new("ls");
 
  match list_cmd.current_dir(dir).status() {
      Ok(cmd) => cmd,
      Err(e) => panic!("Error: {}", e),
  };
 
  println!("\n\n");
}

enum Custom<T, U> {
  EXAMPLE(T),
  SAMPLE(U),
}
struct Rectangle<T> {
  height: T,
  width: T,
}

struct Cube<T, U, V> {
  height: T,
  width: U,
  length: V,
}

fn struct1() {
 
  let rect1 = Rectangle{height: 1, width: 2};
  let rect1 = Rectangle{height: 1.65, width: 2.22};
 
  let cube1 = Cube{height: 1, width: 2, length: 3};
  let cube1 = Cube{height: 1.54, width: 2, length: 3.75};
}

fn sum<T: std::ops::Mul<Output = T>>(num1:T,num2:T) -> T {
  num1 * num2
}

fn lookup_datatype<T>(object:T){
  println!("{}",std::any::type_name::<T>());
}

fn main(){
  println!("{}",sum(1, 2));

  lookup_datatype(1);
  lookup_datatype(1.5);
  lookup_datatype("String");
}
