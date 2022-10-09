fn main() {
    let string = "This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course".to_string();
    let result = concatenate_words(string);
    println!("{}",result);
}

fn concatenate_words(my_string: String) -> String {
    let mut string = "".to_string();
    for found in my_string.split_whitespace() {
        let mut founder = found.to_string();
        let word = &founder[0..1].to_string();
        if word == "c" {
            string.push_str(found);
            string.push(' ');
        }
    }
    string.pop();
    string
}
