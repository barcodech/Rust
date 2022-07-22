enum TimeOfDay {
    Morning,
    Noon,
    Night,
}

fn check_time(input: TimeOfDay) -> impl FnMut(f32) -> f32 {

    use TimeOfDay::*;

    match input {
        Morning => |mut x| {
            x += 3.00;
            println!("Good morning, The time is now {}", x);
            x
        },
        Noon => |mut x| {
            x += 6.00;
            println!("Good noon, The time is now {}", x);
            x
        },
        Night => |mut x| {
            x += 15.00;
            println!("Good night, The time is now {}", x);
            x
        },
    }
}

fn main(){
    use TimeOfDay::*;
    let mut start_time = 6.00;

     let mut closure1 = check_time(Morning);
     let mut closure2 = check_time(Noon);
     let mut closure3 = check_time(Night);

     closure1(start_time);
     closure2(start_time);
     closure3(start_time);
}