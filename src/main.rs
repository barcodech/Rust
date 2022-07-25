fn main() {
    let default_i8: i8 = Default::default();
    let default_str: String = Default::default();
    let default_bool: bool = Default::default();
 
    println!("'{}', '{}', '{}'", default_i8, default_str, default_bool);
}

#[derive(Debug)]
struct  Character{
    name: String,
    age: u8,
    lifestate: Lifestate,
}

#[derive(Debug)]
enum Lifestate {
    Alive,
    Dead,
}

impl Character {
    fn new(name: String, age: u8, lifestate: bool) -> Self {
        Self {
            name,
            age,
            lifestate: if lifestate {
                Lifestate::Alive
            } else {
                Lifestate::Dead
            },
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Adam".to_string(),
            age: 20,
            lifestate: Lifestate::Alive
        }
    }
}

fn main(){
    let character_1 = Character::default();

    println!("The character {:?} is {:?} years old",character_1.name,character_1.age);
}