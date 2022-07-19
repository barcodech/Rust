use std::rc::Rc;
use std::cell::RefCell;

fn add_char(input: Rc<RefCell<String>>){
    let mut my_string2 = input.borrow_mut();
    my_string2.push('!');
    println!("Numbers of owners {}",Rc::strong_count(&input));

}

fn main(){
    let my_string = String::from("I am a string");
    let my_ref = Rc::new(RefCell::new(my_string));
    println!("Numbers of owners {}",Rc::strong_count(&my_ref));
    
    add_char(Rc::clone(&my_ref));
    println!("{:?}",my_ref);

}