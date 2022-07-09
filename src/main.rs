struct Company {
    name: String,
    ceo: Option<String>,
}
 
impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
 
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}
 
fn main() {
    let company_vec = vec![
        Company::new("Amazon", "Jeff Bazos"),
        Company::new("Apple", "Steve Jobs"),
        Company::new("Shopee", ""),
        Company::new("Lazada", ""),
    ];
 
    let all_ceos = company_vec
        .into_iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();
 
    println!("{:?}", all_ceos);
}
 
 
fn main() {
    let user_input = vec!["8.9", "5", "8.0", "8", "10"];
 
    let floating_numbers = user_input
        .into_iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
 
    println!("{:?}", floating_numbers);
}
 
 
struct Company {
    name: String,
    ceo: Option<String>,
}
 
impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
 
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}
 
fn main() {
    let company_vec = vec![
        Company::new("Amazon", "Jeff Bazos"),
        Company::new("Apple", "Steve Jobs"),
        Company::new("Shopee", ""),
        Company::new("Lazada", ""),
    ];
 
    let mut results_vec = vec![];
 
    company_vec
        .iter()
        .for_each(|company| results_vec.push(company.get_ceo().ok_or("No CEO found")));
 
    for item in results_vec {
        println!("{:?}", item);
    }
}
 
 
