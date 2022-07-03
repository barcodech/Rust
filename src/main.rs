use std::collections::VecDeque;

fn finished(input:&mut VecDeque<(&str,bool)>){
    let mut finish_list = input.pop_back().unwrap();
    finish_list.1 = true;
    input.push_front(finish_list);

}

fn unfinished(input:& VecDeque<(&str,bool)>){
    for item in input {
        if item.1 == false {
            println!("Unfinished {} ",item.0);
        }
    }
}

fn main(){
    let mut my_vecqeque = VecDeque::new();
    let tasks = vec!["shooping","dinner","wash a car"];

    for list in tasks {
        my_vecqeque.push_front((list,false));
    }

    finished(&mut my_vecqeque);
    finished(&mut my_vecqeque);

    unfinished(&my_vecqeque);

    for task in my_vecqeque{
        print!("{:?}",task);
    }
}