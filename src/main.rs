struct Foods {
    name:String,
    color:String,
    price:u8,
}

fn main(){
    let fruit = Foods {
        name: "Pineapple".to_string(),
        color:"yellow".to_string(),
        price:50,
    };
    let Foods {
        name: a,
        color: b,
        price: c,
    } = fruit;

    println!("I am eating {}. It is {}. I buy {} baht",a,b,c);
}