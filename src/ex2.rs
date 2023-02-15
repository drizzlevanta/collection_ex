//Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
//so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
//(“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

pub fn demo() {
    let test_pig_latin = String::from("firsttime");

    let pig_latin_test = pig_latin(&test_pig_latin);

    println!("pig latin for {} is {}", test_pig_latin, pig_latin_test);

    let text = "hello apple hello world";
    println!("pig latin for \n {} is: \n {}", text, pig_latin_text(text));
}

fn pig_latin_text(s: &str) -> String {
    //creates an empty string
    let mut pig_latin_text: String = String::new();

    //remove white space and create an iterator
    let mut words = s.trim().split_whitespace().peekable();
    while let Some(word) = words.next() {
        //iterate while there is a word
        pig_latin_text.push_str(&pig_latin(word));
        if let Some(_) = words.peek() {
            //if there is a next word, add space
            pig_latin_text.push(' ');
        }
    }
    pig_latin_text
}

fn pig_latin(word: &str) -> String {
    let mut ch_iter = word.chars().peekable(); //peekable iterator

    let mut s = String::new();
    let mut suffix = None;
    //read until end of string
    while let Some(ch) = ch_iter.next() {
        //if suffix not set, set it
        if let None = suffix {
            suffix = Some(match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    s.push(ch);
                    String::from("-hay")
                }
                _ => ch.to_string() + "ay",
            });
            continue;
        }

        s.push(ch);
    }

    s.push_str(&suffix.unwrap());
    s
}
