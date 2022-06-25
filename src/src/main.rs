fn main() {
    let tuple = ("String", 1, vec!['a'], 'b', [1, 2, 3], 7.7);
    let tuple2 = (5,6);
    println!("{:?}{:?}",tuple,tuple.1);
    println!("{:?}",tuple.1 + tuple2.0);
}
 
fn main() {
    let str_vec = vec!["one", "two", "three"];
 
    let (a, b, _) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{} {}", b,a);
}
 
 
 
