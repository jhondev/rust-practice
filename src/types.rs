pub struct Customer {
    pub name: String,
    pub age: i32,
}

pub type RetirementAgeFn = fn(customer: Customer) -> i32;

pub fn get_user_info(get_retirement_age: &RetirementAgeFn) -> &str {
    let customer = Customer {
        name: "John".to_owned(),
        age: 23,
    };

    match get_retirement_age(customer) {
        50 => "info 1",
        60 => "info 2",
        _ => "info 3",
    }
}
