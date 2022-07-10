fn my_vec(char_vec: &Vec<char>, letter: char) {
    println!("Is {} inside? {}", letter, char_vec.iter().any(|&char| char == letter));
   
}
 
fn main() {
    let char_vec = ('a'..'z').collect::<Vec<char>>();
    my_vec(&char_vec, 'b');
    my_vec(&char_vec, 'c');
    my_vec(&char_vec, 'd');
    println!("All alphabetic? {}", char_vec.iter().all(|&char| char.is_alphabetic()));
   
}
 
fn main() {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
 
    println!("{:?}", num_vec.iter().find(|&number| number % 3 == 0));
    println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));
 
    println!("{:?}", num_vec.iter().position(|&number| number % 3 == 0));
    println!("{:?}", num_vec.iter().position(|&number| number * 2 == 30));
 
}
