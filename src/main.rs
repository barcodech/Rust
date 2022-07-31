#[derive(Clone)]
struct Calculator {
    results: Vec<String>,
    current_input: String,
    total: i32,
    adds: bool,
}

impl Calculator {
    fn new() -> Self {
        Self { 
            results: vec![], 
            current_input: String::new(),
            total:0,
            adds: true,
         }
    }

    fn clear(&mut self) {
        self.current_input.clear();
    }

    fn push_char(&mut self, character: char){
        self.current_input.push(character);
    }
}



const OKEY_CHARACTERS: &str = "123456789+- ";
 
fn math(input: &str) -> i32 {
    if !input.chars().all(|charater| OKEY_CHARACTERS.contains(charater)) ||
    !input.chars().take(2).any(|character| character.is_numeric())
    {
        panic!("Please only input number, +-, or spaces");
    }
 
    let input = input.trim_end_matches(|x| "+- ".contains(x)).chars().filter(|x| *x != ' ').collect::<String>();
 
    let mut cal = Calculator::new();
 
    for character in input.chars() {
        match character {
           
            '+' => {
                if !cal.current_input.is_empty(){
                    cal.results.push(cal.current_input.clone());
                    cal.clear();
                }
            },
            '-' => {
                if cal.current_input.contains('-') || cal.current_input.is_empty(){
                    cal.push_char(character);
                } else {
                    cal.results.push(cal.current_input.clone());
                    cal.clear();
                    cal.push_char(character);
                }
            },
            number => {
                if cal.current_input.contains('-') {
                    cal.results.push(cal.current_input.clone());
                    cal.clear();
                    cal.push_char(number);
                } else {
                    cal.push_char(number);
                }
            },
        }
    }
 
    cal.results.push(cal.current_input);

 
    for entry in cal.results {
        if entry.contains('-') {
            if entry.chars().count() % 2 == 1 {
                cal.adds = match cal.adds {
                    true => false,
                    false => true
                };
                continue;
            } else {
                continue;
            }
        }
        if cal.adds {
            cal.total += entry.parse::<i32>().unwrap();
        } else {
            cal.total -= entry.parse::<i32>().unwrap();
            cal.adds = true;
        }
    }
    cal.total
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
        assert_eq!(math("1 + 2"),3);
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
 
 
