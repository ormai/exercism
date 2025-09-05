pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let i = (student.bytes().next().unwrap() - b'A') as usize * 2;
    diagram
        .lines()
        .flat_map(|line| {
            line[i..i + 2].chars().map(|cup| match cup {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                _ => "violets",
            })
        })
        .collect()
}
