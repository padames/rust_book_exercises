fn main() {
    let mut s1 = String::from("Hello");
    let s2 = s1.clone();
    s1.push_str(" World!!");
    println!("s2={}, world!", s2);
    println!("s2={s2}!");
    let len = calculate_length(&s1);
    println!("The length of the string s1={} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
