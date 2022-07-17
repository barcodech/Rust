use std::fmt::Debug;
struct Dragon {
    boold: i32,
}
#[derive(Debug)]
struct Archer{}
#[derive(Debug)]
struct Wizard{}

trait Frontfight:Debug {
    fn attack_1(&self,opponent:&mut Dragon){
        opponent.boold -= 50;
        println!("You are {:?}",self);
        println!("You use attack1. Dragon now has {} boold left",opponent.boold);
    }

    fn attack_2(&self,opponent:&mut Dragon){
        opponent.boold -= 100;
        println!("You are {:?}",self);
        println!("You use attack2. Dragon now has {} boold left",opponent.boold);
    }
}

impl Frontfight for Archer {}
impl Frontfight for Wizard {}

trait Distancefight:Debug {
    fn magic_1(&self,opponent:&mut Dragon,distance:u32){
        if distance < 10 {
            opponent.boold -= 50;
            println!("You are {:?}",self);
            println!("You use magic1. Dragon now  has {} boold left",opponent.boold);
        }
    }

    fn magic_2(&self,opponent:&mut Dragon,distance:u32){
        if distance > 10 {
            opponent.boold -= 300;
            println!("You are {:?}",self);
            println!("You use magic2. Dragon now  has {} boold left",opponent.boold);
        }
    }
}

impl Distancefight for Wizard {}

fn main(){
    let archer1 = Archer{};
    let wizard1 = Wizard{};

    let mut dragon1 = Dragon {boold:500};

    let distance = 50;

    archer1.attack_1(&mut dragon1);
    wizard1.magic_2(&mut dragon1, distance)
}