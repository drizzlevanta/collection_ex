//improved version
//Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
//so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
//(“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
pub fn demo() {
    let text = "pig latin opt apple hello world";
    println!("pig latin for \n {} is: \n {}", text, pig_latin_text(text));
}

fn pig_latin_text(s: &str) -> String {
    s.trim()
        .split_whitespace()
        .map(pig_latin)
        .collect::<Vec<_>>()
        .join(" ") //notice use of collect
}

fn pig_latin(word: &str) -> String {
    let mut iter = word.chars();
    let first_ch = iter.next();
    match first_ch {
        Some('a' | 'e' | 'i' | 'o' | 'u') => format!("{}-hay", word),
        None => String::new(),
        _ => format!("{}{}-ay", iter.collect::<String>(), first_ch.unwrap()), //using collect to make a String
    }
}
