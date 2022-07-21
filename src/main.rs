use std::rc::Rc;
use std::cell::RefCell;
 
use std::sync::{Arc, Mutex};
 
fn main() {
    let my_number = Rc::new(RefCell::new(0));
 
    for _ in 0..2 {
        let my_number_clone = Rc::clone(&my_number);
        let handle = std::thread::spawn(move || {
            for _ in 0..10 {
                *my_number_clone.borrow_mut() += 1;
            }
        });
        *my_number_clone.borrow_mut() += 1;
                                 
    }
 
   
    println!("{:?}", my_number);
}
 
fn main() {
    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];
 
    for _ in 0..2 {
        let my_number_clone = Arc::clone(&my_number);
        let handle = std::thread::spawn(move || {
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
            }
        });
         handle_vec.push(handle);
        //println!("{:?}", handle.join());                        
    }
 
    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{:?}", my_number);
}
 
 
