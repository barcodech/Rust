fn main() {
    let name = "good morning"; 
    let name2 = String::from("good afternoon"); 
    let name3 = "good morning".to_string();
    println!("{}",name);
}
 
fn main() {

    println!("A String is {:?} bytes",std::mem::size_of::<String>()); 
    println!("i8 is {:?} bytes", std::mem::size_of::<i8>());
    println!("f64 is {:?} bytes", std::mem::size_of::<f64>());
    println!("&str1 is {:?} bytes", std::mem::size_of_val("Hello")); 
    println!("&str2 is {:?} bytes", std::mem::size_of_val("Good morning"));
}

---------------------------strings
fn main() {
    let mut Hello = String::from("Hello ");
    Hello.push('W');
    Hello.push_str("orld");

    println!("Length: {}",Hello.len());
    println!("Capacity: {}",Hello.capacity());
    println!("Is Empty: {}",Hello.is_empty());
    println!("Contains: {}",Hello.contains("Hello"));
    println!("Replace: {}",Hello.replace("World","There"));
    for word in Hello.split_whitespace(){
        println!("{}",word);
    }
    let mut letter = String::with_capacity(4);
    letter.push('a');
    letter.push('b');
    letter.push('c');
   
    assert_eq!(3,letter.len());
    assert_eq!(4,letter.capacity());


    println!("{}",letter);

}