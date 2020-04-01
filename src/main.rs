mod types;

fn main() {
    call_types()
}

fn call_types() {
    let get_retirement_age: types::RetirementAgeFn = |customer| match customer.name.as_str() {
        "John" => 50,
        _ => 70,
    };
    let info = types::get_user_info(&get_retirement_age);
    print!("{}", info)
}
