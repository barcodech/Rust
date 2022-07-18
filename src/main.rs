use std::rc::Rc;
 
#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: Rc<String>,
}
 
#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}
 
fn main() {
    let Singapore = City {
        name: "Singapore".to_string(),
        population: 200_000,
        city_history: Rc::new("Singapore a island country in Southeast Asia.
        ...".to_string()),
    };
 
    let canada_cities = CityData {
        names: vec![Singapore.name],
        histories: vec![Singapore.city_history.clone()],
    };
 
   
    println!("Singapore's history is: {}", Singapore.city_history);
    println!("{}", Rc::strong_count(&Singapore.city_history));
   
    let new_owner = Singapore.city_history.clone();
    println!("{}", Rc::strong_count(&Singapore.city_history));
 
    println!("{:?}", Rc::try_unwrap(Singapore.city_history));
   
}
 
 
