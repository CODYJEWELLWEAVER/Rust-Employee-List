mod employee;
mod ui;
mod map_util;

use ui::Choices;

fn main() {

    let mut user_choice: Choices; 
    let mut map = map_util::new_map();

    loop {
        // display options 
        ui::print_options();
        // and collect option choice
        user_choice = ui::get_user_choice();

        match user_choice {
            Choices::Add => {
                let employee = ui::get_employee();
                map_util::add_employee(employee, &mut map);
            },
            Choices::ListDepartment => {
                let department = ui::get_department();
                map_util::print_department(department, &map);
            },
            Choices::ListAll => {
                map_util::print_all(&map);
            },
            Choices::Quit => {
                ui::quit_prompt();
                break;
            },
            Choices::Invalid => {
                ui::invalid_choice_prompt();
            },
        }
    }
}
