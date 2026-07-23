pub fn brackets_are_balanced(string: &str) -> bool {
    let mut temp: Vec<char>= Vec::new();

    for char in string.chars() {
        if ['{', '(', '['].contains(&char) {
            temp.push(char)
        } else if ['}', ')', ']'].contains(&char) {
            match char {
                '}' => match temp.pop() {
                    Some('{') => continue,
                    _ => return false,
                }
                ')' => match temp.pop() {
                    Some('(') => continue,
                    _ => return false,
                }
                ']' => match temp.pop() {
                    Some('[') => continue,
                    _ => return false,
                },
                _ => { return false }
            }
        } else {
            continue
        }
    }

    temp.is_empty()
}