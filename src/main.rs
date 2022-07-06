use std::convert::From;
#[derive(Debug)]
struct EvenOddVec(Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
    fn from(input: Vec<i32>) -> Self {
        let mut even_odd_vec:Vec<Vec<i32>> = vec![vec![],vec![]];

        for item in input{
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }
        Self(even_odd_vec)
    }
}

fn main(){
    let numbers_vec = vec![1,2,-3,4,-5,6,7];
    let new_vec = EvenOddVec::from(numbers_vec);

    println!("Even{:?}",new_vec);
    
}