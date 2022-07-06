use std::fmt::{Debug,Display};

fn print_it<T:Debug + Display + AsRef<str>>(input: T){
    println!("{}",input)
}

fn main(){

    print_it("string");
    print_it("String".to_string());
}

