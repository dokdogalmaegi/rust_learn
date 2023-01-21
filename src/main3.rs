fn main() {
    let test = String::from("test word");

    println!("{}", find_first_space_index(&test));
    println!("{}", get_first_word(&test));
}

fn find_first_space_index(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    str.len()
}

fn get_first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}