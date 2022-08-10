#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(($x));
            )*
            temp_vec
        }
    };
}
 
fn main()
{
    let _x =vec!["a","b","c"];
    println!("{:?}",_x);
 
}
 
 
use std::collections::HashMap;
 
#[macro_export]
macro_rules! make_map {
    ( $k:expr, $v:expr ) => {
        {
            let mut map = HashMap::new();
            //println!("Key: {}", $k);
            //println!("Value: {}", $v);
            map.insert($k, $v);
            map
        }
    };
}
 
#[macro_export]
macro_rules! make_map_2 {
    ( $( ( $k:expr, $v:expr ) ),* ) => {
        {
            let mut map = HashMap::new();
            $(
                //println!("Key: {}", $k);
                //println!("Value: {}", $v);
                map.insert($k, $v);
            )*
            map
        }
    };
}
 
 
fn main() {
    let map1: HashMap<i32, i32> = make_map!(1, 3);
    println!("{:#?}", map1);
 
    let map2: HashMap<i32, i32> = make_map_2![
        (1, 2),
        (3, 4),
        (5, 6)
    ];
    println!("{:#?}", map2);
}