use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.is_empty() {
        return proverb
    }

    for words in list.windows(2) {
        writeln!(&mut proverb, "For want of a {} the {} was lost.", words[0], words[1]).unwrap();
    }

    // Can't panic. If the list was empty, the function would have returned already.
    write!(&mut proverb, "And all for the want of a {}.", list.first().unwrap());

    proverb
}
