use std::fs::File;
use std::io::prelude::*;
 
fn main() {
  let mut file = File::create("output.txt").expect("Can't create file");
 
  file.write_all(b"Hello Worlds").expect("Can't write")
}
 
 
use std::fs::File;
use std::io::prelude::*;
 
fn main() {
  let mut file = File::open("info.txt").expect("Can't open file");
 
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Can't read file..");
 
  println!("{}",contents);
 
}
 
use std::fs::File;
use std::io::prelude::*;
 
fn main() -> std::io::Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello world")?;  
    let mut hello_file = File::open("hello.txt")?;
    let mut hello_string = String::new();
    hello_file.read_to_string(&mut hello_string)?;
    println!("{}",hello_string);
    Ok(())
}
 
 
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
 
fn main() -> std::io::Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello world\n")?;  
 
    let mut hello_file = OpenOptions::new()
    .append(true)
    .read(true)
    .open("hello.txt")?;
 
    file.write_all(b"Monday\n")?;
    write!(&mut file, "Tuesday\n")?;
    write!(&mut file, "Wednesday")?;
   
    let mut hello_string = String::new();
    hello_file.read_to_string(&mut hello_string)?;
    println!("{}",hello_string);
    Ok(())
}
 
 
 
 
 
 
