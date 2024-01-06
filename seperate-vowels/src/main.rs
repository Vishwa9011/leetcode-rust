/*
6
nrupul


-> uu
-> nrpl
*/

fn main() {
    seperate_vowels("nrupul");
}

fn seperate_vowels(s: &str) -> () {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];

    let mut vowel_str: String = String::new();
    let mut non_vowel_str: String = String::new();

    for (_, val) in s.chars().enumerate() {
        if vowels.contains(&val) {
            vowel_str.push(val);
        } else {
            non_vowel_str.push(val);
        }
    }

    println!("vowelStr: {}", vowel_str);
    println!("non_vowel_str: {}", non_vowel_str);
}
