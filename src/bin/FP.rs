use std::collections::HashMap;




fn main(){
    let ex = ["a","b","c","a","b","a"];
    let counter = ex.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    println!("{:?}", counter);
}