use std::io::{stdin, Error, ErrorKind, Result};
use std::{collections::HashMap, process::exit};
use voca_rs::*;

fn main() {
    let mut hmap: HashMap<&str, &Vec<&str>> = HashMap::new();
    let mut departments: Vec<&str> = Vec::new();
    let mut employees: Vec<&str> = Vec::new();

    loop {
        let command: Vec<&str> = match get_command("Please enter a command:") {
            Ok(c) => c,
            Err(e) => continue,
        };
        do_command(&mut hmap, &mut departments, &mut employees, &command);
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut input).expect("Unable to get input!");
    if input.is_empty() {
        exit(0);
    };
    let input: String = input.trim().to_string();
    input
}

fn get_command(prompt: &str) -> Result<Vec<&str>> {
    let input = get_input(prompt);

    let command = match parse_input(&input) {
        Ok(c) => Ok(c),
        Err(e) => {
            println!("Error: {} Please try again.", e);
            Err(e)
        }
    };
    command
}

fn do_command<'s, 'v>(
    hmap: &mut HashMap<&'s str, &'v Vec<&'s str>>,
    departments: &'v mut Vec<&'s str>,
    employees: &'v mut Vec<&'s str>,
    command: &'v Vec<&'s str>,
) {
    let employee = command[1];
    let department = command[3];
    match command[0] {
        "Add" => {
            // Check for department and employee
            // and add them to the respective vectors
            // before inserting them into the hash map.
            if hmap.contains_key(department) {
                add_employee_to_employees(employees, employee);
            } else {
                add_department_to_departments(departments, department);
                add_employee_to_employees(employees, employee);
                hmap.insert(department, employees);
            }
        }
        "Remove" => {
            if hmap.contains_key(department) {
                remove_employee_from_employees(employees, employee);
            } else {
                println!("No such department {}", department);
            }
        }
        _ => {
            println!("Unknown command {}", command[0]);
        }
    };
}

fn add_employee_to_employees<'s>(employees: &mut Vec<&'s str>, employee: &'s str) {
    match employees.iter().find(|e| *e == &employee) {
        Some(_) => {},
        None => employees.push(employee),
    }
}

fn add_department_to_departments<'s>(departments: &mut Vec<&'s str>, department: &'s str) {
    departments.push(department);
}

fn remove_employee_from_employees<'s>(employees: &mut Vec<&'s str>, employee: &'s str) {
    match employees.iter().position(|e| *e == employee) {
        Some(index) => {employees.remove(index);},
        None => {
            println!("Employee {} is not part of the department.", employee);},
    }
}

fn parse_input(input: &str) -> Result<Vec<&str>> {
    let words = split::words(input);
    for word in words {
        case::capitalize(word, true);
    }
    let action = words[0];
    if (action == "Add") | (action == "Remove") {
        Ok(words)
    } else {
        Err(Error::new(ErrorKind::NotFound, "Command not found"))
    }
}
