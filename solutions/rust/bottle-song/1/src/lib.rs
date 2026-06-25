pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in 0..take_down {
        verses.push(make_verse(start_bottles - i));
    }

    verses.join("\n\n")
}

const WORDS: [&str; 11] = [
    "no",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
];

fn get_current_and_next_words(num: u32) -> (&'static str, &'static str) {
    (
        WORDS[num as usize],
        WORDS[(num - 1) as usize],
    )
}

fn bottle_word(num: u32) -> &'static str {
    if num == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

fn make_verse(num: u32) -> String {
    let (current, next) = get_current_and_next_words(num);
    let next = next.to_ascii_lowercase();

    format!(
        "{current} green {current_bottles} hanging on the wall,\n\
         {current} green {current_bottles} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {next} green {next_bottles} hanging on the wall.",
        current = current,
        current_bottles = bottle_word(num),
        next = next,
        next_bottles = bottle_word(num - 1),
    )
}
