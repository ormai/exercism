pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let east_west_highs: &Vec<_> = &input.iter().map(|row| row.iter().max()).collect();
    let north_south_lows: &Vec<_> = &(0..input[0].len())
        .map(|c| input.iter().map(|r| &r[c]).min())
        .collect();

    (0..input.len())
        .flat_map(|r| {
            (0..input[0].len()).filter_map(move |c| {
                if &input[r][c] >= east_west_highs[r]? && &input[r][c] <= north_south_lows[c]? {
                    Some((r, c))
                } else {
                    None
                }
            })
        })
        .collect()
}
