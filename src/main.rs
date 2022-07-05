
 
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
