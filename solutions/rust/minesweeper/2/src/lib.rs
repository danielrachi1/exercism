pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let num_rows = minefield.len();
    let num_cols = minefield[0].len();

    let annotated_rows: Vec<String> = minefield.iter().enumerate().map(|(row_idx, row_str)| {
        let new_row_string: String = row_str.as_bytes().iter().enumerate().map(|(col_idx, &char_byte)| {
            if char_byte == b'*' {
                '*'
            } else {
                let mut count = 0;
                // Relative coordinates for neighbors
                let deltas: [(i32, i32); 8] = [
                    (-1, -1), (-1, 0), (-1, 1),
                    (0, -1),          (0, 1),
                    (1, -1), (1, 0), (1, 1),
                ];

                for (dr, dc) in deltas {
                    let neighbor_r = row_idx as i32 + dr;
                    let neighbor_c = col_idx as i32 + dc;

                    if neighbor_r >= 0 && neighbor_r < num_rows as i32 &&
                        neighbor_c >= 0 && neighbor_c < num_cols as i32 && 
                        minefield[neighbor_r as usize].as_bytes()[neighbor_c as usize] == b'*' {
                        count += 1;
                    }
                }

                if count > 0 {
                    std::char::from_digit(count as u32, 10).unwrap_or('?')
                } else {
                    ' '
                }
            }
        }).collect::<String>();
        new_row_string
    }).collect::<Vec<String>>();

    annotated_rows
}