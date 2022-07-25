#![allow(dead_code)]
#![allow(unused_variables)]
 
#[derive(Debug)]
struct Struct1 {}
struct Struct2 {}
 
fn main() {
    let char = 'a';
    let str = "Hello";
   
}
 
#[derive(Clone, Copy,Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Number {
    number: i32,
}
 
fn does_nothing(input: Number) {
}
 
fn main() {
    let number = Number {
        number: 8,
    };
    does_nothing(number);
    println!("{:?}",number);
}
 
#[cfg(target_os = "linux")]
#[test]
fn do_nothing() {}
 
#[cfg(target_os = "windows")]
#[test]
fn do_other_thing() {
    assert_eq!(5,10);
}
 
 
 
 
 
