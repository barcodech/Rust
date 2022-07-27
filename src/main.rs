struct Mynum(u8);
fn main() {
    let my_number = Mynum(20);
    println!("{}", *my_number + 20);
}
 
use std::ops::Deref;
#[derive(Debug)]
struct Mynum(u8);
 
impl Deref for Mynum {
    type Target = u8;
 
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
 
fn main() {
    let my_number = Mynum(20);
    println!("{:?}", *my_number + 20); //my_number.pow(1)
}
 
 
use std::ops::{Deref, DerefMut};
struct Mynum(u8);
impl Mynum {
    fn times_two(&self) {
        println!("{}", self.0 * 2);
    }
}
impl Deref for Mynum {
    type Target = u8;
 
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
 
impl DerefMut for Mynum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
 
fn main() {
    let mut my_number = Mynum(20);
    *my_number = 2;
    my_number.times_two();
    println!("{:?}", my_number.pow(1));
}
 
 
use std::ops::Deref;
struct Character {
    name: String,
    health: u8,
    alignment: Alignment,
}
impl Character {
    fn new(
        name: String,
        health: u8,
        alignment: Alignment,
 
    ) -> Self {
        Self {
            name,
            health,
            alignment,
 
        }
    }
}
 
enum Alignment {
    Good,
    Evil,
}
 
impl Deref for Character {
    type Target = u8;
 
    fn deref(&self) -> &Self::Target {
        &self.health
    }
}
fn main() {
    let adam = Character::new("Adam".to_string(), 9, Alignment::Good);
    let jack = Character::new("Jack".to_string(), 10, Alignment::Evil);
 
    let mut health_vec = vec![];
    health_vec.push(*adam);    
    health_vec.push(*jack);  
 
    println!("{:?}", health_vec);
}
 
 
