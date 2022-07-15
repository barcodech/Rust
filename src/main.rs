use std::sync::Mutex;
 
fn main() {
    let my_mutex = Mutex::new(5);
   
    let mut mutex_changer = my_mutex.lock().unwrap();
    //*my_mutex.lock().unwrap() = 6;
 
    println!("{:?}", my_mutex);
 
 
    *mutex_changer = 6;
    std::mem::drop(mutex_changer);
    println!("{:?}", mutex_changer);
 
    println!("{:?}", my_mutex);
}
use std::sync::Mutex;
 
fn main() {
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();
 
    if let Ok(value) = other_mutex_changer {
        println!("The mutex has: {}", value)
    } else {
        println!("Didn't get the lock")
    }
}
use std::sync::Mutex;
 
fn main() {
    let my_mutex = Mutex::new(5);
 
    for _ in 0..100 {
        *my_mutex.lock().unwrap() += 1;
    }
 
    println!("{:?}", my_mutex);
}
 
 
 
