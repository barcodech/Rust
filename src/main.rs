fn main() {
    let days = vec!["Monday", "Sunday"];
 
    let cycle_vec = (0..5)
        .zip(days.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();
    println!("{:?}", cycle_vec);
}
 
fn main() {
    let five_vec1 = ('a'..).take(5).collect::<Vec<char>>();
    let five_vec2 = ('a'..).skip(100).take(5).collect::<Vec<char>>();
 
    println!("{:?}", five_vec1);
    println!("{:?}", five_vec2);
}
 
 
fn main() {
    let numbers_vec = vec![1, 2, 3, 4, 5];
 
    println!("{}", numbers_vec
        .iter()
        .fold(10, |total_so_far, next_number| total_so_far + next_number)
    );
}
 
fn main() {
    let days = "Monday Tuesday Wednesday";
 
    println!("{}",days
            .chars()
            .fold("*-*".to_string(), |mut string_so_far, next_char| {
                string_so_far.push(next_char);
                string_so_far.push('/');
                string_so_far}
            ));
}
 
 
fn main() {
    let num_vec = (0..10)
        .skip_while(|&x| x < 2)
        .take_while(|&x| x < 8)
        .collect::<Vec<_>>();
 
    let clone_vec: Vec<_> = num_vec.iter().cloned().collect();
 
 
    println!("{:?}",clone_vec);
 
    println!("{}",(0..10).sum::<i32>());
}
