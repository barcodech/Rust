use std::collections::BinaryHeap;
fn main() {
    let mut tasks = BinaryHeap::new();
 
    tasks.push((10, "shopping"));
    tasks.push((8, "wash a car"));
    tasks.push((7, "walking dog"));
    tasks.push((6, "dinner"));
    tasks.push((5, "check mails"));
 
    while let Some(tasks) = tasks.pop() {
        println!("You need to: {}", tasks.1);
    }
}
use std::collections::BinaryHeap;
 
fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
                                                         
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number)
    }
    remainder_vec
}
fn main() {
    let numbers_vec = vec![0, 5, 10, 15, 20, 25, 30];
 
    let mut my_heap = BinaryHeap::new();
 
    for number in numbers_vec {
        my_heap.push(number);
    }
 
    while let Some(number) = my_heap.pop() {
        println!("Popped off {}. Remaining numbers are: {:?}", number, show_remainder(&my_heap));
    }
}
