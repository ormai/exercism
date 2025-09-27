use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student = student.to_owned();
        if !self.grades.values().any(|grade| grade.contains(&student)) {
            self.grades.entry(grade).or_default().insert(student);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .map_or_else(Vec::new, |students| students.iter().cloned().collect())
    }
}
