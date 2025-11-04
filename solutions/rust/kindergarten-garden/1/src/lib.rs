//! Get each student's plants.
//!
//! ### Problem Breakdown
//!
//! A diagram will be provided as a `&str`. This will contain two rows,
//! separated by `'\n'`. And each line will contain letters representing
//! the seeds each student has. Every student has two seeds on each line,
//! organized alphabetically.
//! A `&str` with the name of the student will be provided.
//! A `Vec<&str>` with the list of full plant names will be returned.
//!
//!  The provided diagrams can contain up to 12 students.
//!
//! This problem could be solved by manipulating indexes, instead
//! of string slices.
//!
//! ### Single walktrough
//!
//! The list of plants for "Alice" is requested. Alice is index 0 in the
//! students list.
//! Let's assume the diagram has seeds for all 12 students.
//! Alice's seeds are located at index: 0 and 1 of each row. Or chars 0, 1, 25,
//! and 26 of the string slice. Where index 24 is the line break.
//! Bob's (index 1) seeds are located at chars 2, 3, 27, 28.
//! Larry's (index 11) seeds are located at chars 22, 23, 47, 48.
//! Seeds are obtained from the indices, then converted to their full plant
//! names, and returned as a list.
//!
//! ### Generalization
//!
//! The seeds for each student are located at:
//!
//! [2*i, 2*i + 1, 2*i + N*2 + 1, 2*i + N*2 + 2]
//!
//! Where:
//! - N is the number of students in the diagram.
//! - i is the index of the student we want to get the seeds for.
//!
//! ### Algorithm
//!
//! 1. Get the number of students in the diagram.
//! 2. Convert the student name to its index.
//! 3. Get the [2*i, 2*i + 1, 2*i + N*2 + 1, 2*i + N*2 + 2] elements from the diagram.
//! 5. Convert the seeds to full plant names.
//! 5. Return the list with the full plant names.

use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let n_of_students = (diagram.len() - 1) / 4;

    let children = HashMap::from([
        ("Alice", 0),
        ("Bob", 1),
        ("Charlie", 2),
        ("David", 3),
        ("Eve", 4),
        ("Fred", 5),
        ("Ginny", 6),
        ("Harriet", 7),
        ("Ileana", 8),
        ("Joseph", 9),
        ("Kincaid", 10),
        ("Larry", 11),
    ]);

    let plants = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    let student_index = *children.get(student).unwrap();

    let seed_indexes = [
        2*student_index,
        2*student_index + 1,
        2*student_index + n_of_students * 2 + 1,
        2*student_index + n_of_students * 2 + 2,
    ];

    seed_indexes
        .iter()
        .map(|seed_index| {
            *plants
                .get(&diagram.chars().nth(*seed_index).unwrap())
                .unwrap()
        })
        .collect()
}
