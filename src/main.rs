#[derive(Debug)]
struct Employee {
    employee_name: String,
    employee_salary: Option<u32>,
    employee_id: u32,
    employee_type: EmployeeType,
}

#[derive(Debug)]
enum EmployeeType {
    Engineer(String),
}

impl Employee {
    pub fn new(id: u32, name: String, r#type: String) -> Employee {
        let mut new_emp = Employee {
            employee_name: name,
            employee_type: EmployeeType::Engineer(r#type.to_owned()),
            employee_salary: None,
            employee_id: id,
        };

        match r#type.as_str() {
            "Junior" => {
                new_emp.employee_salary = Some(50000);
            }
            "Senior" => {
                new_emp.employee_salary = Some(60000);
            }
            _ => {
                panic!("failed!")
            }
        }

        new_emp
    }
}

fn main() {
    let jr_dev = Employee::new(1, "Kranthi".to_string(), "Junior".to_string());
    let sr_dev = Employee::new(2, "Kiran".to_string(), "Senior".to_string());

    dbg!(&jr_dev);
    dbg!(&sr_dev);
}
