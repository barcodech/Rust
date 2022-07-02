use std::collections::HashSet;
 
fn main() {
    let numbers_vec = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,58, 64, 80, 16, 61, 57, 14, 11];
 
    let mut number_hashset = HashSet::new();
 
    for number in numbers_vec {
        number_hashset.insert(number);
    }
 
    let hashset_length = number_hashset.len();
    println!("There are {} unique numbers, so missing {}.", hashset_length, 100 - hashset_length);
 
    let mut missing_vec = Vec::new();
    for num in 0..100 {
        if number_hashset.get(&num).is_none() {
            missing_vec.push(num);
        }
    }
 
    print!("It does not contain: ");
    for num in missing_vec {
        print!("{} ", num);
    }
}
use std::collections::BTreeSet;
fn main() {
    let numbers_vec = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,58, 64, 80, 16, 61, 57, 14, 11];
 
    let mut number_btreeset = BTreeSet::new();
 
    for number in numbers_vec {
        number_btreeset.insert(number);
    }
    for number in number_btreeset {
        print!("{} ", number);
    }
}
