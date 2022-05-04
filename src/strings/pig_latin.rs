use std::io;

pub fn run() {
    let vowel = vec!["a", "e", "i", "o", "u", "y"];

    for _ in (0..3).rev() {
        println!("please enter any word: ");

        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("error.");
        
        let trimmed_word = word.trim();

        let first_letter = &trimmed_word[0..1];
        let result;

        if vowel.contains(&first_letter) {
            result = format!("{}-{}", trimmed_word, "hay");
        } else {
            result = format!("{}-{}{}", &trimmed_word[1..trimmed_word.len()], &first_letter, "ay");
        }

        println!("pig latin: {}", result);
    }
}