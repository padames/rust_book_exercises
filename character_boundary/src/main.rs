// find English and emoticon boundaries in native String
extern crate emojis;

fn main() {
    let fruit: Vec<_> = emojis::Group::FoodAndDrink.emojis().map(|e| e.as_str()).take(5).collect();
    assert_eq!(fruit, ["🍇", "🍈", "🍉", "🍊", "🍋"]);
    
    let test_string = String::from(fruit[0]);
    let long_fruity_string = "Hello ".to_owned() + &test_string + ", " + fruit[1] + "!"; 
    
    println!("\"{}\"", long_fruity_string);

    for letter in long_fruity_string.chars() {
        println!("{letter}");
    }

    let mut long_fruity_string_in_binary = "".to_string();

    for character in long_fruity_string.clone().into_bytes() {
        long_fruity_string_in_binary += &format!("0{:b} ", character);
    }
    println!(r#""{}" in binary is "{}""#, long_fruity_string, long_fruity_string_in_binary);
}
