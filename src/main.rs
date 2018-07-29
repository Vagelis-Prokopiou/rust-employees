use std::collections::BTreeMap;

mod helpers;

fn main() {
    let mut employees_database: BTreeMap<String, Vec<String>> = BTreeMap::new();

    loop {
        println!();
        println!("What would you like to do?");
        println!("Press 1 to add a new employee.");
        println!("Press 2 to retrieve all existing employees per department.");
        println!("Press 3 to retrieve all existing employees of a specific department.");
        println!("Press q to quit.");
        println!();
        println!("Please provide your selection:");

        let user_action: String = helpers::get_user_input();

        if user_action == "1" {
            println!("Please provide the new employee. E.g.: \"Add George to Sales\"");

            let new_employee: String = helpers::get_user_input();
            let tokens: Vec<&str> = new_employee.trim().split(' ').collect();

            if !helpers::is_valid_employee_provided(&tokens) {
                println!("Wrong format. Try \"Add George to Sales\"");
                continue;
            }

            let employee_name: String = tokens[1].to_owned();
            let employee_department: String = tokens[3].to_owned();

            if employees_database.contains_key(&employee_department) {
                let employees: &mut Vec<String> = employees_database.get_mut(&employee_department).unwrap();
                employees.push(employee_name);
                employees.sort();
            } else {
                employees_database.insert(employee_department, vec![employee_name]);
            }

            println!("{:?}", employees_database);
        } else if user_action == "2" {
            for (key, value) in employees_database.iter() {
                println!("Department: {}", key);
                println!("Employees: {:?}", value);
                println!();
            }
        } else if user_action == "3" {
            println!("Please provide the desired department. The available departments are:");
            println!("{:?}", employees_database.keys());
            let department: String = helpers::get_user_input();
            if !employees_database.contains_key(&department) {
                println!("Wrong department. Please try again.");
                continue;
            }
            println!("Department: {}", department);
            println!("Employees: {:?}", employees_database.get(&department).unwrap());
        } else if user_action == "q" {
            break;
        } else {
            println!("Wrong selection. Please, try again.");
        }
    }
}