fn main() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
 
    for chunk in num_vec.chunks(2) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(2) {
        println!("{:?}", window);
    }
}
 
fn main() {
    let name = "Name1: Adam. Name2: Jack. Name3: Matty.";
    let name_locations = name.match_indices("Name").collect::<Vec<(_, _)>>();
    println!("{:?}", name_locations);
}
 
 
fn main() {
    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable();
 
    for _ in 0..3 {
        println!("the number {}", number_iter.peek().unwrap());
        println!("the number {}", number_iter.peek().unwrap());
       
        number_iter.next();
    }
}
 
 
fn main() {
    let fruits = vec![
        ("orange", 25),
        ("apple", 40),
        ("mango", 35),
        ("watermelon", 200),
    ];
    let mut fruits_iter = fruits.iter().peekable();
    while fruits_iter.peek().is_some() {
        match fruits_iter.peek() {
            Some((name, price)) if price < &100 => {
                println!("cheap friut: {} with {} baht", name, price)
            }
            Some((name, price)) => println!("expensive friut: {} with {} baht", name, price),
            None => break,
        }
        fruits_iter.next();
    }
}
 
 
