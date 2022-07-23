fn main(){
    let my_box = Box::new(10);
    let interger = *my_box;
    println!("{:?}",my_box);
    println!("{:?}",interger);
}

use std::error::Error;
use std::{fmt, vec};

#[derive(Debug)]
struct  ErrorOne;
impl Error for ErrorOne {}
impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!")
    }
}

#[derive(Debug)]
struct  ErrorTwo;
impl Error for ErrorTwo {}
impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

fn return_errors(input: u8) -> Result<String,Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine".to_string()),
    }
}

fn main(){

    let num_vec = vec![0,1,10];
    for number in num_vec {
        match return_errors(number) {
            Ok(massage) => println!("{}",massage),
            Err(massage) => println!("{}",massage),
        }
    }
}