
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    s = takes_ownership_and_gives_back(s);
    println!("{} {}", s, calculate_length(&s));
    let word = first_word(&s);
    println!("{}", word );
}

fn takes_ownership_and_gives_back(some_string: String) -> String { 
    println!("{}", some_string);
    some_string
} 

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
