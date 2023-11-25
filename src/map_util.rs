use std::collections::HashMap;
use crate::employee::Employee;

pub fn new_map() -> HashMap<String, Vec<Employee>> {
    HashMap::new()
}

pub fn add_employee(
    empl: Employee, 
    map: &mut HashMap<String, Vec<Employee>>
) {
    let dept = String::from(empl.get_dept());
    
    // gets vector for department
    let dept_vector = map.entry(dept).or_insert(Vec::new());

    // insert employee, dept_vector takes ownership 
    dept_vector.push(empl);
}

pub fn print_department(
    dept: String,
    map: &HashMap<String, Vec<Employee>>
) {
    let dept_vector = map.get(&dept);
    if let Some(vector) = dept_vector {
        println!("Employees in {}:", &dept);
        for empl in vector {
            empl.print();
        }
    } else {
        println!("No employees in {}!", &dept);
    }
}

pub fn print_all(
    map: &HashMap<String, Vec<Employee>>
) {
    println!("Employees:");
    for dept in map.keys() {
        print_department(dept.to_string(), &map); 
    }
}
