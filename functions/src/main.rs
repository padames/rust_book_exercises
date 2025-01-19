fn another_function(_a_value:i32,unit_lable: String) {
    println!("I am in 'another_function', saying {}", unit_lable)
}

fn main() {
    println!("Testing a function call");
    another_function(35, String::from("hello"));
}

