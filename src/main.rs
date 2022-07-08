fn main(){

    let my_vec = vec![1,2,3];

    let index = my_vec.get(4).unwrap_or_else(|| {
        if my_vec.get(4).is_some(){
            &my_vec[2]
        } else {
            &0
        }
    }

    );

    println!("{}",index);
}