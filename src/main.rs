fn return_str() -> String {
    let country = String::from("Malaysia");
    let country_ref = &country;
    //println!("{}",country_ref)
    country_ref.to_string()
}
 
fn main() {
    let country = return_str();
    println!("{}",country)
}
 
fn main() {
    let country = String::from("Malaysia");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);
}
 
fn main() {
    let mut number = 10;
    let number_ref = &number;
    let number_change = &mut number;
    *number_change += 10;
   
    println!("{}", number_change);
}
 
fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name
}
 
fn main() {
    let country = String::from("Malaysia");
    let country2 = print_country(country);
    print_country(country2);
}
 
 
fn print_country(country_name: &String) {
    println!("{}", country_name);
}
 
fn main() {
    let country = String::from("Malaysia");
    print_country(&country);
    print_country(&country);
}
 
fn add_city(country_name: &mut String) {
    country_name.push_str("-Penang");
    println!("Welcome to {}", country_name);
}
 
fn main() {
    let mut country = String::from("Malaysia");
    add_city(&mut country);
}
 
fn add_city(mut country_name: String) {
    country_name.push_str("-Penang");
    println!("Welcome to {}", country_name);
}
 
fn main() {
    let country = String::from("Malaysia");
    add_city(country);
}
 
 
 
