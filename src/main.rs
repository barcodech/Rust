fn four_operations(input: f64) {
    println!("number {}:
floor: {}
ceiling: {}
rounded: {}
truncated: {}\n",
        input,
        input.floor(),
        input.ceil(),
        input.round(),
        input.trunc()
    );
}
 
fn main() {
    four_operations(2.3);
    four_operations(10.5);
    four_operations(-1.7);
    four_operations(-18.8);
}
 
 
fn main() {
    let my_vec = vec![8.0_f64, 6.8, 5.4, 11.0, 23.0, 50.35, 50.2, 30.2, -9.44, -18.0];
    let maximum = my_vec.iter().fold(f64::MIN, |current_number, next_number| current_number.max(*next_number));
    let minimum = my_vec.iter().fold(f64::MAX, |current_number, next_number| current_number.min(*next_number));
    println!("{}, {}", maximum, minimum);
}
 
 
fn main() {
    let boolean = (true, false);
    println!("{} {}", boolean.0 as u8, boolean.1 as u8);
}
 
 
fn main() {
    let boolean: (u16, u16) = (true.into(), false.into());
    println!("{} {}", boolean.0, boolean.1);
}
 
fn main() {
    let (boo1, boo2) = (true.then(|| 10), false.then(|| 10));
    println!("{:?}, {:?}", boo1, boo2);
}
 
 
fn main() {
    let bool_vec = vec![true, false, false, true];
   
    let option_vec = bool_vec
        .iter()
        .map(|item| {
            item.then(|| {
                println!("Got a {}!", item);
                "It's true"
            })
        })
        .collect::<Vec<_>>();
 
    println!("Now we have: {:?}", option_vec);
 
    let filtered_vec = option_vec.into_iter().filter_map(|item| item).collect::<Vec<_>>();
 
    println!("without the Nones: {:?}", filtered_vec);
}
 
 
 
