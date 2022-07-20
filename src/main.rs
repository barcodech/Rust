fn main() {
    std::thread::spawn(|| {
        println!("I am printing a");
    });
    std::thread::spawn(|| {
        println!("I am printing b");
    });
}
fn main() {
    for _ in 0..5 {
        let handle = std::thread::spawn(|| {
            println!("I am printing a");
        });
        let handle1 = std::thread::spawn(|| {
            println!("I am printing b");
        });
        handle.join();
        handle1.join();
    }
}
 
fn main() {
    for i in 0..5 {
        let handle = std::thread::spawn(move || {
            println!("I am printing {}",i);
        });
       
        handle.join();
    }
}
 
 
fn main() {
    for i in 0..5 {
        let handle = std::thread::spawn(move || {
            if i % 2 == 0{
                panic!()
            } else {
                100
            }
        });
        println!("{:?}",handle.join());
       
    }
}
 
fn main() {
    let mut handle_vec = vec![];
 
    for i in 0..5 {
        let handle = std::thread::spawn(move || {
            println!("I am printing {}",i);
        });
        handle_vec.push(handle);
    }
    for handle in handle_vec {
        handle.join().unwrap();
    }
}
 
 
