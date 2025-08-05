#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let sublist = is_sublist(first_list, second_list);
    let superlist = is_sublist(second_list, first_list);

    if sublist & superlist {
        Comparison::Equal
    } else if sublist {
        Comparison::Sublist
    } else if superlist {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist(first_list: &[i32], second_list: &[i32]) -> bool {
    if first_list.is_empty() {
        return true;
    }
    if first_list.len() > second_list.len() {
        return false;
    }

    for window in second_list.windows(first_list.len()) {
        if window == first_list {
            return true;
        }
    }

    false
}
