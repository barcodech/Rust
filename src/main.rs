fn main() {
    let my_cities = ["bangkok", "pattaya", "phuket"];
 
    for city in my_cities {
        println!("{}", city);
    }
    for city in &my_cities {
        println!("{}", city);
    }
    for city in my_cities.iter() {
        println!("{}", city);
    }
 
    let [city1, city2, city3] = my_cities;
    println!("{}", city1);
 
}
 
fn main() {
    let word = "มะพร้าว";
    for character in word.chars() {
        print!("{} ", character.escape_unicode());
    }
}
 
fn main() {
    let some_number = 100_u8;
    let other_number = 100_u8;
 
    println!("{:?}", some_number.checked_add(other_number));
    println!("{:?}", some_number.checked_add(1));
}
