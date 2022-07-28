mod box1{
  fn toy(){
    println!("I am playing");
  }

  pub fn apple(){
    println!("I am eating");
  }

  pub mod box2{
    pub fn movie(){
      println!("I am watching");
    }
  }
}
  
fn main() {
  box1::box2::movie();
}

mod print_things {
    use std::fmt::Display;
 
    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input)
    }
}
 
fn main() {
    use crate::print_things::prints_one_thing;
 
    prints_one_thing(10);
    prints_one_thing("Print something...".to_string());
}
 
 
mod print_things {
    use std::fmt::{Display, Debug};
 
    pub enum Choises {
        Good,
        Bad,
    }
 
    #[derive(Debug)]
    pub struct Copy {
        name: String,
        pub times_to_print: u32,
    }
 
    impl Copy {
        pub fn new(times_to_print: u32) -> Self {
            Self {
                name: "Copy".to_string(),
                times_to_print,
            }
        }
 
        pub fn print_copy(&self) {
            for _ in 0..self.times_to_print {
                println!("{:?}", self.name);
            }
        }
    }
 
    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input)
    }
}
 
fn main() {
    use crate::print_things::*;
 
    let my_copy = Copy::new(5);
    my_copy.print_copy();
}
 
 
mod country {
    fn print_country(country: &str) {
        println!("We are in the country of {}", country);
    }
    pub mod province {
        fn print_province(province: &str) {
            println!("in the province of {}", province);
        }
 
        pub mod city {
            use super::super::*;
            use super::*;        
 
            pub fn print_city(country: &str, province: &str, city: &str) {
                print_country(country);
                print_province(province);
                println!("in the city of {}", city);
            }
        }
    }
}
 
fn main() {
    use crate::country::province::city::print_city;
 
    print_city("Thailand", "Bangkok", "Bangna");
    print_city("Thailand", "Chonburi", "Pattaya");
}
 
 
 
