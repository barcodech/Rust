Rust,Unix UTF-8 Windows UTF-16
 
fn main() {
pub fn into_string(self) -> Result<String, OsString>
}
 
use std::ffi::OsString;
 
fn main() {
    let os_string = OsString::from("This also works for OS.");
    match os_string.into_string() {
        Ok(valid) => valid.cloney(),          
        Err(not_valid) => not_valid.cloney(),  
    }
}
 
use std::mem;
 
fn main() {
    println!("{}", mem::size_of::<i32>());
    let my_array = [8; 50];
    println!("{}", mem::size_of_val(&my_array));
    let mut some_string = String::from("You can drop a String because it's on the heap");
    mem::drop(some_string);
}
 
 
use std::{mem};
 
#[derive(Debug)]
struct Food{
    name1: String,
    name2: String,
}
 
fn main() {
    let mut fruits = Food{
        name1: "orage".to_string(),
        name2: "mango".to_string()
    };
    println!("{:?}", fruits);
    mem::swap(&mut fruits.name1, &mut fruits.name2);
    println!("{:?}", fruits);
    mem::replace(&mut fruits.name1, "apple".to_string());
    println!("{:?}", fruits);
    let (fruit3,fruit4) = (mem::take(&mut fruits.name1),mem::take(&mut fruits.name2));
    println!("{:?}", fruits);
    println!("{} {}", fruit3,fruit4);
}
 
 
