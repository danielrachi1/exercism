use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.into_iter().flat_map(|base_value| get_multiples_up_to(*base_value, limit)).collect::<HashSet<_>>().iter().sum()
}

fn get_multiples_up_to(n: u32, limit: u32) -> Vec<u32> {
    let mut multiples = Vec::new();
    let mut multiplier = 1;
    
    while n*multiplier < limit {
        multiples.push(n*multiplier);
        multiplier += 1;
    }

    multiples
}