use std::io;
use crate::employee::Employee;

#[derive(Debug)]
pub enum Choices {
    Add,
    ListDepartment,
    ListAll,
    Quit,
    Invalid,
}

pub fn print_options() {
    println!("Choose an option:");
    println!("  1: Add employee");
    println!("  2: List department employees");
    println!("  3: List all employees");
    println!("  4: Quit ('4' or 'q')");
}

pub fn get_user_choice() -> Choices {
    let mut raw_input = String::new();

    io::stdin().read_line(&mut raw_input)
        .expect("Cannot read user choice!");

    let input = raw_input.trim();

    match &input[..] {
        "1" => Choices::Add,
        "2" => Choices::ListDepartment,
        "3" => Choices::ListAll,
        "4" | 
        "q" => Choices::Quit,
         _  => Choices::Invalid, 
    }
}

pub fn quit_prompt() {
    println!("Goodbye.");
}

pub fn invalid_choice_prompt() {
    println!(
        "You have entered an invalid choice, please try again!"
    );
}

pub fn get_employee() -> Employee {
    let mut name_in = String::new();
    let mut dept_in = String::new();

    // get employee name from user
    println!("Enter a name for the employee:");
    io::stdin().read_line(&mut name_in)
            .expect("Cannot read name!");

    let empl_name = name_in.trim().to_string();

    // get employee dept
    println!("Enter {}'s department:", empl_name);
    io::stdin().read_line(&mut dept_in)
            .expect("Cannot read department!");

    let empl_dept = dept_in.trim().to_string();

    println!("Adding {} to {}!", empl_name, empl_dept);

    Employee::new(empl_name, empl_dept)
}

pub fn get_department() -> String {
    let mut dept_in = String::new();

    println!("Enter the name of the department:");

    io::stdin().read_line(&mut dept_in)
                .expect("Cannot read department name!");

    let dept = dept_in.trim().to_string();

    dept
}
