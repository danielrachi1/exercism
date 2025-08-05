#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist(potential_sublist: &[i32], list_to_check: &[i32]) -> bool {
    potential_sublist.is_empty() || 
    list_to_check
        .windows(potential_sublist.len())
        .any(|window| window == potential_sublist)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let first_is_sub_of_second = is_sublist(first_list, second_list);
    let second_is_sub_of_first = is_sublist(second_list, first_list);

    match (first_is_sub_of_second, second_is_sub_of_first) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}