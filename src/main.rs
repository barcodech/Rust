use std::time::Instant;
 
fn main() {
    let time = Instant::now();
    println!("{:?}", time);
}
 
use std::time::Instant;
fn main() {
    let time1 = Instant::now();
    let time2 = Instant::now();
    let mut new_string = String::new();
    loop {
        new_string.push('A');
        if new_string.len() > 10_000 {
            break;
        }
    }
    let time3 = Instant::now();
    println!("{:?}", time2 - time1);
    println!("{:?}", time3 - time1);
}
 
use std::time::Instant;
 
fn random_number(digits: usize) {
    if digits > 10 {
        panic!("Random number can only be up to 10 digits");
    }
    let now = Instant::now();
    let output = format!("{:?}", now);
 
    output
        .chars()
        .rev()
        .skip(3)
        .take(digits)
        .for_each(|character| print!("{}",character));
    println!();
}
 
fn main() {
    random_number(5);
    random_number(3);
    random_number(3);
}
 
use std::time::Duration;
use std::thread::sleep;
 
fn main() {
    let three_seconds = Duration::from_secs(3);
    println!("I must sleep now...");
    sleep(three_seconds);
    println!("Bye..");
}
 
fn main() {
    let true_or_false = true;
 
    match true_or_false {
        true => println!("It's true"),
        false => println!("It's false"),
        true => println!("It's true"),
    }
}
