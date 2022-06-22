fn times_two(number: i32) -> i32 {
    number * 2
}

fn main() {
    
    let final_number = {
        let y = 2;
        let x = 3; 
        let x_twice = times_two(x); 
        let x_twice_and_y = x_twice + y; 
        x_twice_and_y 
    };
    println!("The number is now: {}", final_number)
}
