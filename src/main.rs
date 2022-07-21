/* 
FnOnce - take value and drop it
FnMut - can modify it
Fn - can take by reference 
 */

 fn main(){

    let mut my_string = String::from("simple test");

    let refs = || println!("{}",my_string);
    refs();

    let mut mutes = || {
        my_string.push('!');
        println!("{}",my_string);
    };
    mutes();

    let drops = || drop(my_string);
    drops();

 }
