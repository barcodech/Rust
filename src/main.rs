fn main() {
    let my_number = 15; 
    let reference1 = &my_number; 
    let reference2 = &&my_number; 
    let references3 = &&&my_number;

    println!("{}",***references3 == my_number);
}
