use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let mut word_sorted_lowercase = word.to_lowercase().chars().collect::<Vec<_>>();
    word_sorted_lowercase.sort();

    for anagram in possible_anagrams {
        // handle "sTOp" != "StoP"
        if word.to_lowercase() == *anagram.to_lowercase() {
            continue
        }

        let mut anagram_sorted_lowercase = anagram.to_lowercase().chars().collect::<Vec<_>>();
        anagram_sorted_lowercase.sort();

        if word_sorted_lowercase == anagram_sorted_lowercase {
            result.insert(anagram);
        }
    }

    result
}
