const OKEY_CHARACTERS: &str = "123456789+- ";
 
fn math(input: &str) -> i32 {
    if !input.chars().all(|charater| OKEY_CHARACTERS.contains(charater)) ||
    !input.chars().take(2).any(|character| character.is_numeric())
    {
        panic!("Please only input number, +-, or spaces");
    }
 
    let input = input.trim_end_matches(|x| "+- ".contains(x)).chars().filter(|x| *x != ' ').collect::<String>();
 
    let mut result_vec = vec![];
    let mut spoon = String::new();
 
    for character in input.chars() {
        match character {
           
            '+' => {
                if !spoon.is_empty(){
                    result_vec.push(spoon.clone());
                    spoon.clear();
                }
            },
            '-' => {
                if spoon.contains('-') || spoon.is_empty(){
                    spoon.push(character)
                } else {
                    result_vec.push(spoon.clone());
                    spoon.clear();
                    spoon.push(character);
                }
            },
            number => {
                if spoon.contains('-') {
                    result_vec.push(spoon.clone());
                    spoon.clear();
                    spoon.push(number);
                } else {
                    spoon.push(number);
                }
            },
        }
    }
 
    result_vec.push(spoon);
 
    println!("{}",input);
    println!("{:?}",result_vec);
 
    5
}
 
fn main(){
    math("7 +25+++--");
    println!();
    math("7  --- ++--  --25-");
    println!();
    math("-9+5");
 
}
 
mod tests {
    use super::*;
 
    #[test]
    fn test1() {
        assert_eq!(math("1 + 1"),2);
    }
 
    #[test]
    fn test2() {
        assert_eq!(math("1 + 2"),-1);
    }
   
    #[test]
    fn test3() {
        assert_eq!(math("1 - -1"),2);
    }
 
    #[test]
    #[should_panic]
    fn test4() {
        assert_eq!(math("10 + ten"),10);
    }
 
}    
 
 
