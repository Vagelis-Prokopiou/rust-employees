/*
    Create a text interface to allow a user to add employee names to a department in a company.
    For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    Then let the user retrieve a list of all people in a department
    or all people in the company by department,
    sorted alphabetically.
*/

use std::collections::HashMap;
use std::io;

pub fn get_user_input() -> String {
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");
    return user_input.trim().to_string();
}

pub fn is_valid_employee_provided(vector: &Vec<&str>) -> bool {
    return vector.len() == 4 && vector[0] == "Add" && vector[2] == "to";
}

fn main() {
    let mut employees_database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!();
        println!("What would you like to do?");
        println!("Press 1 to add a new employee.");
        println!("Press 2 to retrieve all existing employees per department.");
        println!("Press 3 to retrieve all existing employees of a specific department.");
        println!("Press q to quit.");
        println!();
        println!("Please provide your selection:");
        let user_selection: String = get_user_input();

        if user_selection == "1" {
            println!("Please provide the new employee. E.g.: \"Add George to Sales\"");

            let new_employee: String = get_user_input();
            let tokens: Vec<&str> = new_employee.trim().split(' ').collect();

            if !is_valid_employee_provided(&tokens) {
                println!("Wrong format. Try \"Add George to Sales\"");
                continue;
            }

            let employee_name: String = tokens[1].to_owned();
            let employee_department: String = tokens[3].to_owned();

            if employees_database.contains_key(&employee_department) {
                let employees: &mut Vec<String> = employees_database.get_mut(&employee_department).unwrap();
                employees.push(employee_name);
            } else {
                employees_database.insert(employee_department, vec![employee_name]);
            }

            println!("{:?}", employees_database);
        } else if user_selection == "2" {
            println!("2 was selected");
        } else if user_selection == "3" {
            println!("3 was selected");
        } else if user_selection == "q" {
            break;
        } else {
            println!("Wrong selection. Please, try again.");
        }
    }
}


