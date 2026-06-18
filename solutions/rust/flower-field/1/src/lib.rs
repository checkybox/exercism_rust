pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for row_index in 0..garden.len() {
        let mut annotated_row = String::new();
        for col_index in 0..garden[row_index].len() {
            if garden[row_index].as_bytes()[col_index] == b'*' {
                annotated_row.push('*');
                continue;
            }

            let flower_count = check_neighbors(garden, row_index, col_index);
            if flower_count == 0 {
                annotated_row.push(' ');
            } else {
                annotated_row.push_str(&flower_count.to_string());
            }
        }
        result.push(annotated_row);
    }

    result
}

fn check_neighbors(garden: &[&str], row_index: usize, col_index: usize) -> u32 {
    let mut count = 0;

    for delta_row in -1i32..=1 {
        for delta_col in -1i32..=1 {
            // skip the center cell itself
            if delta_row == 0 && delta_col == 0 {
                continue;
            }

            let neighbor_row = (row_index as i32 + delta_row) as usize;
            let neighbor_col = (col_index as i32 + delta_col) as usize;

            // bound check
            if neighbor_row < garden.len() && neighbor_col < garden[neighbor_row].len() {
                if garden[neighbor_row].as_bytes()[neighbor_col] == b'*' {
                    count += 1;
                }
            }
        }
    }

    count
}
