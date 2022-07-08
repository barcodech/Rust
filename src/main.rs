use std::collections::HashMap;


fn main(){

    let number_vec = vec![0,1,2,3,4,5];
    let word_vec = vec!["zero","one","two","three","four","five"];

    let hash_vec = number_vec
    .into_iter()
    .zip(word_vec.into_iter())
    .collect::<HashMap<_,_>>();

    for (key,value) in hash_vec {
        println!("{} {}",key,value);
    }
}