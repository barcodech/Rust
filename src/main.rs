fn main() {
    let mut my_vec = vec![100, 50, 80, 2, 4, 6, 0, 0];
    my_vec.sort();
    println!("{:?}", my_vec);
}
 
fn main() {
    let mut my_vec = vec!["apple", "apple", "apple", "orange", "orange"];
    my_vec.dedup();
    println!("{:?}", my_vec);
}
 
fn main() {
    let mut my_vec = vec!["apple", "apple", "apple", "orange", "orange"];
    my_vec.sort();
    my_vec.dedup();
    println!("{:?}", my_vec);
}
 
fn main() {
    let mut push_string = String::new();
    let mut capacity_counter = 0;
    for _ in 0..10 {
        if push_string.capacity() != capacity_counter {
            println!("{}", push_string.capacity());
            capacity_counter = push_string.capacity();
        }
        push_string.push_str("ABCDEFGHIJ");
    }
}
 
fn main() {
    let mut push_string = String::with_capacity(100);
    let mut capacity_counter = 0;
    for _ in 0..10 {
        if push_string.capacity() != capacity_counter {
            println!("{}", push_string.capacity());
            capacity_counter = push_string.capacity();
        }
        push_string.push_str("ABCDEFGHIJ");
    }
}
 
fn main() {
    let mut push_string = String::with_capacity(100);
    let mut capacity_counter = 0;
    for _ in 0..100 {
        if push_string.capacity() != capacity_counter {
            println!("{}", push_string.capacity());
            capacity_counter = push_string.capacity();
        }
        push_string.push_str("ABCDEFGHIJ");
    }
    push_string.shrink_to_fit();
    println!("{}", push_string.capacity());
    push_string.push('K');
    println!("{}", push_string.capacity());
    push_string.shrink_to_fit();
    println!("{}", push_string.capacity());
}
 
fn main() {
    let mut my_string = String::from("ABCDEFGHIJ");
    loop {
        let pop_result = my_string.pop();
        match pop_result {
            Some(character) => print!("{}", character),
            None => break,
        }
    }
}
 
fn main() {
    let mut my_string = String::from("ABC 123 DEF");
    my_string.retain(|character| character.is_alphabetic() || character == ' ');
    dbg!(my_string);
}
 
 
 
