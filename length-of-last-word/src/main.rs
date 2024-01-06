fn main() {
    println!(
        "length of last word is = {}",
        length_of_last_word(String::from("   fly me   to   the moon  "))
    );

    println!(
        "length of last word is = {}",
        length_of_last_word(String::from("Hello World"))
    );
}

fn length_of_last_word(s: String) -> i32 {
    let trimmed_str: &str = s.trim();
    println!("trimmed_str = '{}'", trimmed_str);

    let length = trimmed_str.len() as i32 - 1;

    for i in (0..length).rev() {
        // println!("i = {}", s.chars().nth(i as usize).unwrap());
        // println!("i = {}", s.clone().into_bytes()[i as usize] as char);
        if s.chars().nth(i as usize).unwrap() == ' ' {
            return length - i;
        }
    }

    // println!("trimmed_str = {}", length);

    return 0;
}

fn length_of_last_word(s: String) -> i32 {
    let trimmed_str = s.trim(); // Trim the string to remove leading and trailing whitespaces.
    let length = trimmed_str.len() as i32; // Corrected length calculation.

    for i in (0..length).rev() {
        if trimmed_str.chars().nth(i as usize).unwrap() == ' ' {
            return length - i - 1;
        }
    }

    length
}
