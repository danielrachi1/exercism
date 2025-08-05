use std::collections::HashMap;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let counts = count_adjacent_flowers(garden);
    build_annotated_garden(garden, &counts)
}

fn count_adjacent_flowers(garden: &[&str]) -> HashMap<(usize, usize), u32> {
    let directions = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
    let mut counts = HashMap::new();
    
    for (i, row) in garden.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch == '*' {
                for (dx, dy) in directions {
                    if let Some((ni, nj)) = get_valid_neighbor(garden, i, j, dx, dy) {
                        if get_char_at(garden, ni, nj) != '*' {
                            *counts.entry((ni, nj)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    
    counts
}

fn get_valid_neighbor(garden: &[&str], i: usize, j: usize, dx: i32, dy: i32) -> Option<(usize, usize)> {
    let new_i = i as i32 + dx;
    let new_j = j as i32 + dy;
    
    if new_i >= 0 && new_j >= 0 {
        let ni = new_i as usize;
        let nj = new_j as usize;
        
        if ni < garden.len() && nj < garden.get(ni)?.len() {
            Some((ni, nj))
        } else {
            None
        }
    } else {
        None
    }
}

fn get_char_at(garden: &[&str], i: usize, j: usize) -> char {
    garden[i].chars().nth(j).unwrap_or(' ')
}

fn build_annotated_garden(garden: &[&str], counts: &HashMap<(usize, usize), u32>) -> Vec<String> {
    garden.iter().enumerate().map(|(i, row)| {
        row.chars().enumerate().map(|(j, ch)| {
            match ch {
                '*' => '*',
                _ => counts.get(&(i, j))
                          .and_then(|&cnt| char::from_digit(cnt, 10))
                          .unwrap_or(' ')
            }
        }).collect()
    }).collect()
}