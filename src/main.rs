use std::num::ParseIntError;


fn parse_str(input:&str) -> Result<i32,ParseIntError>{
    let intnumber = input.parse::<u32>()?.to_string().parse::<u8>()?.to_string().parse::<i32>()?;
    Ok(intnumber)
}

fn main(){
    let str_vec = vec!["123","30","9.0","hello","goes"];
    for item in str_vec {
        let output = parse_str(item);
        println!("{:?}",output);
    }
}