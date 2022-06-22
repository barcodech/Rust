fn main() {
   
    print!("\t Good morning\nGood morning");
}
fn main() {
   
    println!("Good morning
Good morning
Good morning
Good morning.");
 
    println!("When I was putting
    together this site of games.");
}
 
fn main() {
    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file.");
    println!(r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#)
}
 
 
fn main() {
    println!("{:?}", b"This will look like numbers");
}
 
fn main() {
    println!("{:X}", 'a' as u32);
    println!("{:X}", 'H' as u32);
    println!("{:X}", 'ก' as u32);
    println!("{:X}", 'ฉ' as u32);
 
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");
}
 
fn main() {
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);
}
 
fn main() {
    let number = 555;
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", number, number, number);
}
 
fn main() {
    let father_name = "adam";
    let son_name = "jack";
    let family_name = "matin";
    println!("This is {1} {2}, son of {0} {2}.", father_name, son_name, family_name);
}
 
fn main() {
    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Bangkok",
        city2 = "Krabi",
        city3 = "Tokyo",
        country = "Thailand"
    );
}
 
fn main() {
    let letter = "a";
    println!("{:*^10}", letter);
}
 
fn main() {
    let title = "MORNING";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "BANGKOK";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
 
 
 
 
