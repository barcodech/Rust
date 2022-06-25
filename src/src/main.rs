fn main() {
    let name1 = String::from("shabu");
    let name2 = String::from("shushi");
 
    let mut my_vec = Vec::new();
   
 
    my_vec.push(name1);
    my_vec.push(name2);
}
 
fn main() {
    let my_vec = vec![7, 8, 9];
    println!("{:?}",my_vec);
}
 
fn main() {
    let mut num_vec = Vec::with_capacity(5);
    num_vec.push('a');
    println!("{}", num_vec.capacity());
   
}
 
fn main() {
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [4, 5, 6].into();
}
 
 
 


use std::mem;
fn main () {
    let mut numbers = vec![1,2,3,4];
    numbers[0] = 0;
    numbers.push(5);
    numbers.push(6);
    numbers.pop();

    println!("{:?}",numbers);
    println!("{}",numbers[3]);
    println!("Length: {}",numbers.len());
    println!("Size: {} bytes",mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}",slice);

    for x in numbers.iter(){
        println!("Number: {}",x);
    }

    for x in numbers.iter_mut(){
        *x += 2;
    }
    println!("Numbers Vector: {:?}",numbers);
}