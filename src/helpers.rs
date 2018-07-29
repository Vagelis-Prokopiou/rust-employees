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
