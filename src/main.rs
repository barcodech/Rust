fn main() {
    let new_vec = vec![1, 5, 10, 0];
     
    let mut empty_vec = vec![];  
 
 
    for index in 0..5 {
        empty_vec.push(
            new_vec
               .get(index)
                .and_then(|number| Some(number + 1))
                .and_then(|number| Some(number + 2))
        );
    }
    println!("{:?}", empty_vec);
}
 
 
fn main() {
    let one = true;
    let two = false;
    let three = true;
    let four = true;
 
    println!("{}", one && three);
    println!("{}", one && two && three && four);
}
 
 
fn main() {
    let first = vec![
         
        None,
        Some("success!"),
        Some("success!"),
        Some("success!"),
       
    ];
    let second = vec![
       
        Some("success!"),
        None,
        Some("success!"),
        Some("success!"),
       
    ];
    let third = vec![
         
        Some("success!"),
        Some("success!"),
        None,
        Some("success!"),
       
    ];
 
    for i in 0..first.len() {
        println!("{:?}", first[i].and(second[i]).and(third[i]));
    }
}
