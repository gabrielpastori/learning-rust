use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Type the word to be converted to pig latin");

    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    word = word.trim().to_string();
    
    let mut letters: Vec<_> = word.graphemes(true).collect();
    convert_to_pig_latin(&mut letters);    
    let converted_word = letters.concat();

    println!("The converted word is {}", converted_word);
}

fn convert_to_pig_latin(letters: &mut Vec<&str>){
    let vowels = "aeiou";
    let mut first_letter = "";

    if let Some(value) = letters.first() {
        first_letter = value;
    }

    if vowels.contains(first_letter) {
        letters.push("h");
        letters.push("a");
        letters.push("y");
    } else {
        letters.remove(0);
        letters.push(first_letter);
        letters.push("a");
        letters.push("y")
    }
}