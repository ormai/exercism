#[rustfmt::skip]
const DIRECTIONS: [(isize, isize); 8] = [
    (-1,  1), (0,  1), (1,  1),
    (-1,  0),          (1,  0),
    (-1, -1), (0, -1), (1, -1),
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(r, &line)| {
            line.chars()
                .enumerate()
                .map(|(c, cell)| {
                    if cell == ' '
                        && let Some(n) = neighbors(garden, r, c)
                    {
                        char::from_digit(n, 10).unwrap()
                    } else {
                        cell
                    }
                })
                .collect()
        })
        .collect()
}

fn neighbors(garden: &[&str], r: usize, c: usize) -> Option<u32> {
    let count = DIRECTIONS
        .iter()
        .filter_map(|&(dr, dc)| {
            if let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                garden
                    .get(nr)
                    .and_then(|&line| line.get(nc..nc + 1))
                    .take_if(|v| *v == "*")
            } else {
                None
            }
        })
        .count();
    if count > 0 { Some(count as u32) } else { None }
}
