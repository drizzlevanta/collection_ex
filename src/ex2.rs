//Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
//so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
//(“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

pub fn demo() {
    let test_pig_latin = String::from("firsttime");

    let pig_latin = pig_latin(&test_pig_latin);

    println!("pig latin for {} is {}", test_pig_latin, pig_latin);
}

fn pig_latin(word: &str) -> String {
    let mut ch_iter = word.chars().peekable(); //peekable iterator
                                               //ignore the first char
                                               // let first_ch = pig.next().unwrap();
                                               // pig.push('a');
                                               // pig.push('y');

    //find first char
    // let first_ch = ch_iter.peek().unwrap();
    // let suffix = match first_ch {
    //     'a' | 'e' | 'i' | 'o' | 'u' => String::from("hay"),
    //     _ => {
    //         //skip first char
    //         ch_iter.next();
    //         first_ch.to_string() + "ay"
    //     }
    // };

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
