use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::process;
use std::string::String;


use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct Record {
    company_id: String,
    region: String,
    car: String,
    order_date: String,
}

impl Record {
    fn get_id(&self) -> &String {
        return &self.company_id;
    }
}

impl Borrow<String> for Record {
    fn borrow(&self) -> &String {
        return &self.company_id;
    }
}


fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(io::stdin());
    let mut vec: Vec<Record> = vec![];
    for result in rdr.deserialize() {
        //vec = result?.iter().map(parse_line).collect::<Vec<&str>>();
        let res: Record = result?;
        vec.push(res);
    };
    let order_map = count_orders_per_company(vec);
    let mut sorted: Vec<_> = order_map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", sorted);
    return Ok(());
}


fn count_orders_per_company(orders: Vec<Record>) -> HashMap<String, i32>{
    let mut orders_per_company = HashMap::new();

    for order in orders {
        let id = order.get_id().as_str();
        if orders_per_company.contains_key(id) {
            let idx = &orders_per_company.get_mut(id);
            match idx {
                Some(val) => {
                    let newval = **val;
                    _ = orders_per_company.insert(id.to_owned(), newval + 1)
                },
                None => println!("this should never happend"),
            }
        } else {
            orders_per_company.insert(id.to_owned(), 1);
        }
    };
    return orders_per_company;
}

/*
fn parse_line(line: &str) -> &String {
    let (id, _) = line.split_once(";").expect("must contain ;");
    return id.to_string();
}
*/

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
