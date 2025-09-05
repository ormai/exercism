const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (1, 1),
    (0, 1),
    (1, -1),
    (1, 0),
    (-1, -1),
    (0, -1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield: Vec<Vec<u8>> = minefield
        .iter()
        .map(|row| row.as_bytes().to_vec())
        .collect();
    for r in 0..minefield.len() {
        for c in 0..minefield[r].len() {
            if minefield[r][c] == b' ' {
                let count = DIRECTIONS.iter().fold(0, |acc, &(dr, dc)| {
                    if let Some(&val) = get(&minefield, dr + r as isize, dc + c as isize) {
                        if val == b'*' {
                            return acc + 1;
                        }
                    }
                    acc
                });
                if count > 0 {
                    minefield[r][c] = count + 48;
                }
            }
        }
    }
    minefield
        .iter()
        .map(|row| String::from_utf8_lossy(row).into_owned())
        .collect()
}

fn get<T>(grid: &[Vec<T>], r: isize, c: isize) -> Option<&T> {
    if r >= 0 && c >= 0 {
        let (r, c) = (r as usize, c as usize);
        if r < grid.len() && c < grid[r].len() {
            return Some(&grid[r][c]);
        }
    }
    None
}
