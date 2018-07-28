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
    let mut employees_database: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut user_input: String = String::new();

    loop {
        println!("What would you like to do?");
        println!("1 to add a new employee.");
        println!("2 to retrieve all existing employees.");
        println!("3 to retrieve all existing employees per department");
        println!("Please provide your input:");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        let tokens: Vec<&str> = user_input.trim().split(' ').collect();
        let employee_name: &str = tokens[1];
        let employee_department: &str = tokens[3];

        if employees_database.contains_key(employee_department) {
            println!("employees_database.contains_key");
            let employees: &mut Vec<&str> = employees_database.get_mut(employee_department).unwrap();
            employees.push(employee_name);
        } else {
            println!("NOT employees_database.contains_key");
            employees_database.insert(employee_department, vec![employee_name]);
        }


        /*{
            let employees: &mut Vec<&str> = employees_database.entry(department).or_insert(vec![]);
            employees.push(name);
        }

        println!("{:?}", user_input);
        println!("{:?}", employees_database);*/
        break;
    }
}
