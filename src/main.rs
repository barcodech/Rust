#[derive(Debug)]
struct  Names{
    one_word:Vec<String>,
    two_words:Vec<String>,
    three_words:Vec<String>,
}

fn main() {

    let vec_of_names = vec![
        "Monday Tuesday Thursday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday Friday",
        "Saturday",
        "Sunday",
    ];

    let mut iter_names = vec_of_names.iter().peekable();

    let mut all_names = Names {
        one_word:vec![],
        two_words:vec![],
        three_words:vec![],
    };

    while iter_names.peek().is_some(){
        let next_item = iter_names.next().unwrap();
        match next_item.match_indices(' ').count() {
            0 => all_names.one_word.push(next_item.to_string()),
            1 => all_names.two_words.push(next_item.to_string()),
            _ => all_names.three_words.push(next_item.to_string()),
            
        }
    }
    println!("{:?}",all_names);
    
}