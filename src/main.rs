fn main(){
    let num_vec = vec![2,3,4];

    num_vec
    .iter()
    .enumerate()
    .for_each(|(index,number)| println!("Index {} NUmber {}"
    ,index,number));

    
}