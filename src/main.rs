use std::fmt::Display;
 
fn print_vec<T: Display>(input: &Vec<T>) {
    for item in input {
        print!("{} ", item);
    }
    println!();
}
 
fn main() {
 
    let array_vec = Vec::from([1, 2, 3]);
    print_vec(&array_vec);
 
    let str_vec = Vec::from("string slide");
    print_vec(&str_vec);
 
    let string_vec = Vec::from("string".to_string());
    print_vec(&string_vec);
}
 
 
#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}
 
impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}
#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}
 
impl From<Vec<City>> for Country {
                                 
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}
 
impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}.", city.name, city.population);
        }
    }
}
 
fn main() {
    let johor = City::new("johor", 543_210);
    let penang = City::new("penang", 321_654);
 
    let malay_cities = vec![johor, penang];
    let malay = Country::from(malay_cities);
 
    malay.print_cities();
}
