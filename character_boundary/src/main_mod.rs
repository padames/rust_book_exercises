// find English and emoticon boundaries in native String
extern crate emojis;

fn main() {
    let tets_word = "नमस्ते";
    println!("\"{}\"", tets_word);

    for letter in tets_word.chars() {
        println!("{letter}");
    }
}