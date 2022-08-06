use std::mem::transmute;
 
fn main(){
 
    let thing1 = unsafe { transmute::<[u8; 2], u16>([50,100])};
    let thing2 = unsafe { transmute::<u16, [u8; 2]>(789)};
    let thing3 = unsafe { transmute::<[u64; 2], String>([10,20])};
 
    println!("{} {:?} {}",thing1,thing2,thing3);
}
 
#![no_implicit_prelude]
fn main() {
    let my_vec = vec![1, 2, 3];
    let my_string = String::from("Hello world");
    println!("{:?}, {}", my_vec, my_string);
}
 
#![no_implicit_prelude]
extern crate std;
use std::vec;
use std::string::String;
use std::convert::From;
use std::println;
 
fn main() {
    let my_vec = vec![1, 2, 3];
    let my_string = String::from("Hello world");
    println!("{:?}, {}", my_vec, my_string);
}
 
 
 
