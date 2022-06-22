fn main() {
    let my_number = 15; 
    let reference1 = &my_number; 
    let reference2 = &&my_number; 
    let references3 = &&&my_number;

    println!("{}",***references3 == my_number);
}

---------------------------pointer_ref
fn main() {
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Value: {:?}",(arr1,arr2));

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Value: {:?}",(&vec1,vec2));
}
