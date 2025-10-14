pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut m = vec![vec![0; size as usize]; size as usize];
    let (mut x, mut y) = (0isize, 0isize);
    let (mut dx, mut dy) = (1isize, 0isize);
    for n in 1..=size * size {
        m[y as usize][x as usize] = n;
        if m.get((y - dy) as usize)
            .and_then(|r| r.get((x + dx) as usize))
            .is_none_or(|&n| n != 0)
        {
            (dx, dy) = (dy, -dx) // 90° clockwise rotation
        }
        x += dx;
        y -= dy;
    }
    m
}
