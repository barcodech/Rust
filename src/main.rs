fn get_length(input:String){
    println!("It's {} words long",input.split_whitespace().count());
}

fn main(){
    let mut my_string = String::new();
    for _i in 0..10 {
        my_string.push_str("good morning ");
        get_length(my_string.clone());
    }
}