enum Option<T> {
    None,
    Some(T),
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn check_num(number: i32) -> Result<i32, String> {
    match number {
        10 => Ok(number),
        _ => Err("Sorry, that wasn't ten.".to_string()),
    }
}
fn main() {
    let  result = check_num(5).unwrap();
    println!("{:?}", result);
}
fn main() {
    let mut result_vec = Vec::new();
    for number in 1..=5 {
        result_vec.push(check_num(number));
    }
    println!("{:?}", result_vec);
}
use std::num::ParseIntError;
fn check_num(number:&str) -> Result<i32,ParseIntError> {
    number.parse::<i32>()
}
 
fn main(){
    let my_vec = vec!["5","hello","10"];
 
    for item in my_vec {
        match check_num(item) {
            Ok(number) => println!("Got {}",number),
            Err(e) => println!("{}",e)
        }
    }
}
