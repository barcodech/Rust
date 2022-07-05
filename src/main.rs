 
use std::fmt::Debug;
struct Dragon {
    blood: i32,
}
#[derive(Debug)]
struct Archer {
    blood: i32,
}
#[derive(Debug)]
struct Wizard {
    blood: i32,
}
 
trait Frontfight{}
trait Distancefight{}
impl Distancefight for Wizard {}
impl Frontfight for Archer {}
impl Frontfight for Wizard {}
 
fn attack_1<T: Frontfight + Debug>(character:&T, opponent: &mut Dragon) {
    opponent.blood -= 50;
    println!("you are {:?}",character);
    println!(
        "You use attack1. Dragon now has {} blood left.",
        opponent.blood
    );
   
}
fn attack_2<T: Frontfight + Debug>(character:&T,opponent: &mut Dragon) {
    opponent.blood -= 100;
    println!("you are {:?}",character);
 
    println!(
        "You use attack2. Dragon now has {} blood left.",
        opponent.blood
    );
}
fn magic_1<T: Distancefight + Debug>(character:&T, opponent: &mut Dragon, distance: u32) {
    if distance < 10 {
        opponent.blood -= 50;
        println!("you are {:?}",character);
 
        println!(
            "You use magic1. Dragon now has {} blood left.",
            opponent.blood
        );
    }
}
fn magic_2<T: Distancefight + Debug>(character:&T, opponent: &mut Dragon, distance: u32) {
    if distance > 10 {
        opponent.blood -= 300;
    }
    println!("you are {:?}",character);
 
    println!(
        "You use magic2. Dragon now has {} blood left.",
        opponent.blood
    );
}
 
fn main() {
    let archer1 = Archer {blood: 200};
    let wizard1 = Wizard {blood: 100};
 
    let mut dragon1 = Dragon { blood: 500 };
    let mut dragon2 = Dragon { blood: 600 };
    let distance = 50;
 
 
    attack_1(&archer1,&mut dragon1);
    magic_2(&wizard1,&mut dragon2, distance);
}
