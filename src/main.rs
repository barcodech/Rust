fn main() {
    let my_vec = vec![2, 3, 4];
 
    for index in 0..10 {
      match my_vec.get(index) {
        Some(number) => println!("The number is: {}", number),
        None => {}
      }
    }
}
 
fn main() {
    let my_vec = vec![2, 3, 4];
 
    for index in 0..10 {
      if let Some(number) = my_vec.get(index) {
        println!("The number is: {}", number);
      }
    }
}
 
 
fn main() {
    let vec_foods = vec![
        vec!["KFC", "cola", "1", "-2", "3"],
        vec!["burger", "coffee", "2", "3", "4"],
    ];
    for mut set in vec_foods {
        println!("For the set of {}:", set[0]);
        while let Some(list) = set.pop() {
           
            if let Ok(number) = list.parse::<i32>() {
               
                println!("The number is: {}", number);
            }  
        }
    }
}
