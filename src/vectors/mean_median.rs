use rand::Rng;

pub fn run() {
    let mut vector: Vec<u32> = Vec::new();

    for _ in (0..88).rev() {
        vector.push(rand::thread_rng().gen_range(0..199));
    }

    let mut total_sum = 0;

    for element in vector.iter() {
        total_sum += element;
        print!(" {} ", element);
    }

    vector.sort();

    let count = vector.len();
    let median;

    if count % 2 == 0
    {
        median = count / 2 - 1;
    }
    else
    {
        median = (count - 1) / 2;
    }

    println!("\n");
    println!("------------");
    println!("total sum {}", total_sum);
    println!("median {}", vector[median]);
    println!("------------");
}