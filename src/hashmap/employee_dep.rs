use std::io;
use std::collections::HashMap;

pub fn run() {
    println!("----------------");
    println!("Employee Department");
    println!("----------------");

    let mut departments:HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Add employees to departments (command: 'Add Employee_Name to Department_Name'):");
        let mut add_command = String::new();

        io::stdin().read_line(&mut add_command).expect("Error.");
        let mut command_iter = add_command.split_whitespace();
 
        if command_iter.clone().count() != 4 {
            println!("Wrong input '{}', try one more time", add_command);
            continue;
        }

        command_iter.next();

        let employee = match command_iter.next() {
            Some(v) => v.to_string(),
            None => String::new()
        };
        
        command_iter.next();

        let department = match command_iter.next() {
            Some(v) => v.to_string(),
            None => String::new()
        };

        println!("{} - {}", employee, department);

        departments.entry(department).or_insert(Vec::new()).push(employee);

        if break_cycle("Do you want to add another employee? [y/n]:") {
            break;
        }        
    }

    for (dep, employees) in &departments {
        println!("department: {}", dep);

        for empl in employees {
            println!("employee: {}", empl);
        }
    }
}

fn break_cycle(ask: &str) -> bool {
    println!("{}", ask);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let input = input.trim();

    if !input.to_lowercase().eq("y") {
        return true;
    } 

    false
}