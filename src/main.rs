#[derive(Debug)]
struct  City {
    name: String,
    years: Vec<u32>,
    populations: Vec<u32>,
}

impl City {
    fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {
        Self { 
            name: name.to_string(), 
            years, 
            populations, 
        }
    }
    fn city_data<F>(&mut self, f:F)
    where
        F: Fn(&mut Vec<u32>,&mut Vec<u32>),
    {
        f(&mut self.years,&mut self.populations)
    }
}

fn main(){
    let years = vec![2001,2002,2003,2004,2005];
    let populations = vec![100_000,200_000,300_000,400_000,500_000];

    let mut pattaya = City::new("Pattaya",years,populations);

    pattaya.city_data(|city_years,city_populations|{
        let new_vec = city_years
            .into_iter()
            .zip(city_populations.into_iter())
            .skip(3).take(2)
            .collect::<Vec<(_,_)>>();
        println!("{:?}",new_vec);
    });

    pattaya.city_data(|x,y|{
        x.push(2006);
        y.push(600_000);
    });

    pattaya.city_data(|x,y|{
        let position_option = x.iter().position(|x| *x == 2004);
        if let Some(position) = position_option {
            println!("Going to delete {} at position {:?}",x[position],position);
            x.remove(position);
            y.remove(position);
        }
    });

    println!("Years left are {:?}\nPopulation left are {:?}",pattaya.years,pattaya.populations);

}