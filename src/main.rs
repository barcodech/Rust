use std::fmt;
use std::ops::Add;

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Country {
    fn new(name:&str,population:u32,gdp:u32) -> Self {
        Self { name: name.to_string(),
               population, 
               gdp, 
            }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, other:Self) -> Self {
        Self { name: format!("{} and {}",self.name,other.name), 
            population: self.population + other.population, 
            gdp: self.gdp + other.gdp }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

fn main(){
    let Thailand = Country::new("Thailand",500_000,900_000);
    let Singapore = Country::new("Singapore",400_000,800_000);
    let Maylaysia = Country::new("Maylasia",300_000,700_000);

    
    println!("{}",Thailand.clone());
    println!("{}",Thailand.clone() + Singapore.clone());
    println!("{}",Thailand + Singapore + Maylaysia);
}











