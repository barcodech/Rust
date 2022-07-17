use std::sync::RwLock;
use std::mem::drop;

fn main(){

    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    let read3 = my_rwlock.read().unwrap();

    println!("{:?} {:?} {:?}",read1,read2,read3);
    drop(read1);
    drop(read2);
    drop(read3);


    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 10;

    println!("{:?}",write1);

    println!("{:?}",my_rwlock);
}