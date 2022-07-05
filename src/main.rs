struct Dog{
  name: String
}

struct Cat {
  name: String
}

struct Rat {
  name: String
}

trait Eating {
    fn eating_food(&self) {
      println!("drink water")
    }
}

impl Eating for Dog {
    fn eating_food(&self) {
        println!("{} is eating dog foods",self.name)
    }
}

impl Eating for Cat {
  fn eating_food(&self) {
      println!("{} is eating cat foods",self.name)
  }
}

impl Eating for Rat {}


fn main() {
  let dog1 = Dog {
    name: String::from("Scooby")
  };

  dog1.eating_food();

  let cat1 = Cat {
    name: String::from("Tom")
  };

  cat1.eating_food();

  let rat1 = Rat {
    name: String::from("Jerry")
  };

  rat1.eating_food();

}

use std::fmt::Debug;
struct Dragon {
    blood: i32,
}
#[derive(Debug)]
struct Archer {}
#[derive(Debug)]
struct Wizard {}
 
trait Frontfight: Debug {
    fn attack_1(&self, opponent: &mut Dragon) {
        opponent.blood -= 50;
        println!("you are {:?}",self);
        println!(
            "You use attack1. Dragon now has {} blood left.",
            opponent.blood
        );
       
    }
    fn attack_2(&self,opponent: &mut Dragon) {
        opponent.blood -= 100;
        println!(
            "You use attack2. Dragon now has {} blood left.",
            opponent.blood
        );
    }
}
impl Frontfight for Archer {}
impl Frontfight for Wizard {}
 
trait Distancefight {
    fn magic_1(&self, opponent: &mut Dragon, distance: u32) {
        if distance < 10 {
            opponent.blood -= 50;
            println!(
                "You use magic1. Dragon now has {} blood left.",
                opponent.blood
            );
        }
    }
    fn magic_2(&self, opponent: &mut Dragon, distance: u32) {
        if distance > 10 {
            opponent.blood -= 300;
        }
        println!(
            "You use magic2. Dragon now has {} blood left.",
            opponent.blood
        );
    }
}
impl Distancefight for Wizard {}
 
fn main() {
    let archer1 = Archer {};
    let wizard1 = Wizard {};
 
    let mut dragon1 = Dragon { blood: 500 };
    let mut dragon2 = Dragon { blood: 600 };
    let distance = 50;
 
 
    archer1.attack_1(&mut dragon1);
    wizard1.magic_2(&mut dragon2, distance);
}
