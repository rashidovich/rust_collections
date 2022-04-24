use std::collections::HashMap;

fn main() {
    let mut array = 
        vec![3,12,345,32,2,3,534,645,62,234,453,46,6,32,4266,89,463,63,34,22,53,25,25,36,22,52,26,1];
        //vec![3,1,7,2,7];

    let mut total_sum = 0;
    let mut map:HashMap<String,i32> = HashMap::new();

    for i in &array {
        println!("{}", i);
        total_sum += *i;

        let count = map.entry((*i).to_string()).or_insert(0);
        *count += 1;
    }

    array.sort();

    let count = array.len();
    let median;

    if count % 2 == 0
    {
        median = count / 2 - 1;
    }
    else
    {
        median = (count - 1) / 2;
    }

    println!("------------");
    println!("total sum {}", total_sum);
    println!("median {}", array[median]);
    println!("{:?}", map);
}
