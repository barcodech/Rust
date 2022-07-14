use std::cell::RefCell;


#[derive(Debug)]
struct User {
    username: String,
    active: RefCell<bool>,
}

fn main(){
    let user_1 = User {
        username: "user 1 ".to_string(),
        active: RefCell::new(true),
    };

    let mut borrow_one = user_1.active.borrow_mut();
    *borrow_one = false;
    std::mem::drop(borrow_one);

    *user_1.active.borrow_mut() = false;

    dbg!(&user_1);

}