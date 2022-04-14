fn main() {
    let sentence = "Hello world";
    let word = first_word(sentence);
    println!("{}", word);

    let something = &sentence[0..5];
    println!("{}", something);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
