/* pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
 {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
 
fn main() {} */

use std::borrow::Cow;

fn test(input: u8) -> Cow<'static,str>{

    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}",remainder).into(),
    }
}

fn main(){
    for number in 1..=5 {
        match test(number) {
            Cow::Borrowed(message) => println!("{} The Cow is borrowed {}",number,message),
            Cow::Owned(message) => println!("{} The Cow is owned {}",number,message),

        }
    }
}