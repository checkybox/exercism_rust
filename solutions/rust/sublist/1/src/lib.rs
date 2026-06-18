#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        return Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        return Comparison::Superlist
    } else {
        return Comparison::Unequal
    }
}

fn is_sublist(candidate: &[i32], container: &[i32]) -> bool {
    if candidate.is_empty() {
        return true;
    }
    if candidate.len() > container.len() {
        return false;
    }

    // container = 5, candidate = 2
    // 0..=3, i = 3 -> check 3,4,5
    for i in 0..=(container.len() - candidate.len()) {
        if &container[i..i + candidate.len()] == candidate {
            return true;
        }
    }
    false
}
