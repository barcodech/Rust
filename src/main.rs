fn main() {
    let numbers = "324165465456484";
 
    for (index, number) in numbers.char_indices() {
        match (index % 4, number) {
            (0..=2, number) => print!("{}", number),
            _ => print!("\t"),
        }
    }
}
 
 
fn main() {
    let my_vec = vec![8, 9, 10];
 
    my_vec.iter().for_each(|_| println!("No variables"));
}
 
 
fn main() {
    let months = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
 
    let filtered_months = months
        .into_iter()                        
        .filter(|month| month.len() > 5)    
        .filter(|month| month.contains("e"))
        .collect::<Vec<&str>>();
 
    println!("{:?}", filtered_months);
}
 
fn main(){
    let mut my_vec = vec![1,2,3,4,5];
    my_vec.retain(|item| item % 2 == 0);
    println!("{:?}",my_vec);
}`
