/*
    Create a text interface to allow a user to add employee names to a department in a company.
    For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    Then let the user retrieve a list of all people in a department
    or all people in the company by department,
    sorted alphabetically.
*/

use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees_database: HashMap<String, Vec<String>> = HashMap::new();
    let mut user_input: String = String::new();

    loop {
        println!("What would you like to do?");
        println!("Press 1 to add a new employee.");
        println!("Press 2 to retrieve all existing employees per department.");
        println!("Press 3 to retrieve all existing employees of a specific department.");
        println!("Press q to quit.");
        println!();
        println!("Please provide your selection:");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        let tokens: Vec<&str> = user_input.trim().split(' ').collect();
        let employee_name: String = tokens[1].to_owned();
        let employee_department: String = tokens[3].to_owned();

        if employees_database.contains_key(&employee_department) {
            println!("employees_database.contains_key");
            let employees: &mut Vec<String> = employees_database.get_mut(&employee_department).unwrap();
            employees.push(employee_name);
        } else {
            println!("NOT employees_database.contains_key");
            employees_database.insert(employee_department, vec![employee_name]);
        }

        println!("{:?}", employees_database);
        break;
    }
}
