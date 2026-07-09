const STUDENTS: [&str; 12] = ["Alice", "Bob", "Charlie",
    "David", "Eve", "Fred",
    "Ginny", "Harriet", "Ileana",
    "Joseph", "Kincaid", "Larry"];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let start = STUDENTS
        .iter()
        .position(|&s| s == student)
        .expect("unknown student")
        * 2;

    diagram
        .lines()
        .flat_map(|row| row[start..=start+1].chars())
        .map(plant_name)
        .collect()
}

fn plant_name(c: char) -> &'static str {
    match c {
        'C' => "clover",
        'G' => "grass",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("unknown plant"),
    }
}