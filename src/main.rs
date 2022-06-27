fn main() {
    let mut count = 0;

    loop {
        count +=1;
        println!("Number: {}",count);
        if count == 20 {
            break;
        }
    }

    while count <= 20 {
        if count % 2 == 0 {
            println!("two");
        } else if  count % 3 == 0 {
            println!("three");
        } else if  count % 11 ==0 {
            println!("eleven");
        } else {
            println!("{}",count);
        }
        count +=1;
    }

    for x in 10..20 {
        if x % 2 == 0 {
            println!("two");
        } else if  x % 3 == 0 {
            println!("three");
        } else if  x % 11 ==0 {
            println!("eleven");
        } else {
            println!("{}",x);
        }
    }

  }

  fn main() {
    let mut counter = 0;
    let mut counter2 = 0;
 
    'first_loop: loop {
        println!("The counter is now: {}", counter);
        counter += 1;
        if counter > 5 {
            println!("Start the second loop");
            'second_loop: loop {              
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}
 
fn main() {
    let mut counter = 0;
 
    while counter < 5 {
        counter +=1;
        println!("The counter is now: {}", counter);
    }
}
 
fn main() {
    for _number in 0..3 {
        println!("The number is: {}", number);
    }
 
    for number in 0..=3 {
        println!("The next number is: {}", number);
    }
}
 
fn main() {
    let mut counter = 0;
    let my_number = loop {
        counter +=1;
        if counter * 5 == 40 {
            break counter;
        }
    };
    println!("{}", my_number);
}
 
fn match_colours(rbg: (i32, i32, i32)) {
   
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")];
    let mut goodvalue = true;
    for item in new_vec {
        if item.0 < 20 {
            goodvalue = false;
            println!("Not much {}.", item.1)
        }
    }
    if goodvalue {
        println!("Each colour has good value")
    }
    println!();
}
 
fn main() {
    let first = (255, 0, 0);
    let second = (150, 150, 150);
    let third = (0, 0, 255);
 
    match_colours(first);
    match_colours(second);
    match_colours(third);
}
 
 
 
