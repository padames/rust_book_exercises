// find English and emoticon boundaries in native String
extern crate emojis;
extern crate print_bytes;
extern crate env;

use print_bytes::println_lossy;

fn main() {
    let fruit: Vec<_> = emojis::Group::FoodAndDrink.emojis().map(|e| e.as_str()).take(5).collect();
    assert_eq!(fruit, ["🍇", "🍈", "🍉", "🍊", "🍋"]);
    
    let test_string = String::from(fruit[0]);
    let long_fruity_string = "Hello".to_owned() + &test_string + ", " + fruit[1]; 
    
    // println!("Hello, {}!", long_fruity_string);

    for letter in long_fruity_string.chars() {
        println!("{letter}");
    }



    print!("exe: ");
    println_lossy(&env::current_exe()?);
    println!();

    println!("args:");
    for arg in env::args_os().skip(1) {
        println_lossy(&arg);
    }

    // println_lossy(&test_string);

}
