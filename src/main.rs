macro_rules! input_num {
    (10) => {
        10
    };
    () => {
        println!("You didn't give me any number.");
    };
}
 
fn main() {
    let my_number = input_num!(10);
    println!("{}",my_number);
   
    input_num!();
}
 
 
macro_rules! print_out {
    ($input:expr) => {
        println!("You gave me: {:?}", $input);
    }
}
 
fn main() {
    print_out!("[1,2,3]");
}
 
macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}
 
fn main() {
    let x = 10;
    let my_vec = vec![1, 2, 3];
    check!(x, 10);
    check!(my_vec, vec![1, 2, 3]);
    check!(x, 20);
}
 
 
macro_rules! print_anything {
    ($($input:tt),*) => {
        let output = stringify!($($input),*);
        println!("{}", output);
    };
}
 
 
fn main() {
    print_anything!(dfhsftg, fsdgdfgd);
    print_anything!();
    print_anything!(dfgdfg, sdf, [1,2,5], 097);
}
 
 
 
