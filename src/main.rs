
use std::mem;
fn main () {
    let mut numbers = [1,2,3,4];
    numbers[3] = 5;
    println!("{:?}",numbers);
    println!("{}",numbers[3]);
    println!("Length: {}",numbers.len());
    println!("Size: {} bytes",mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}",slice);
}

fn main() {
    let my_array = [2_u8; 10];
    println!("{:?}", my_array);
}



fn main(){
    let arr = [1,2,3,4,5,6,7,8,9,10];
    let two_five = &arr[1..5];
    let three_ten = &arr[2..];
    let one_five = &arr[..5];
    let everything = &arr[..];

    println!("two_five: {:?}, three_ten: {:?}, one_five: {:?}, everything: {:?}",two_five,three_ten,one_five,everything)
}