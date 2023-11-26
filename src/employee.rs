#[derive(Debug)]
#[derive(Ord)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(PartialEq)]
pub struct Employee {
    name: String,
    department: String,
}

impl Employee {
    /* Public Constructor */
    pub fn new (
        empl_name: String, 
        empl_dept: String
    ) -> Employee {
        Employee {name: empl_name, department: empl_dept}
    }

    pub fn get_dept(&self) -> &str {
        &self.department[..]
    }

    // utility for printing lists in map_util.rs
    pub fn print(&self) {
        println!(
            "\tName: {}", self.name
        );
    }
}
