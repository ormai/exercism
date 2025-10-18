use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut teams: Vec<_> = match_results
        .lines()
        .flat_map(|line| {
            let split: Vec<_> = line.split(';').collect();
            let (t1, t2) = match split[2] {
                "win" => ((1, 0, 0), (0, 0, 1)),
                "draw" => ((0, 1, 0), (0, 1, 0)),
                _ => ((0, 0, 1), (1, 0, 0)),
            };
            vec![(split[0], t1), (split[1], t2)]
        })
        .fold(HashMap::new(), |mut map, (n, (w, d, l))| {
            let e = map.entry(n).or_insert((0, 0, 0, 0, 0));
            *e = (e.0 + 1, e.1 + w, e.2 + d, e.3 + l, e.4 + (w * 3 + d));
            map
        })
        .into_iter()
        .collect();
    teams.sort_by_key(|&(name, (_, _, _, _, points))| (-points, name));
    format!("{:30} | MP |  W |  D |  L |  P", "Team")
        + teams
            .into_iter()
            .map(|(name, (mp, w, d, l, p))| {
                format!("\n{name:30} | {mp:2} | {w:2} | {d:2} | {l:2} | {p:2}")
            })
            .collect::<String>()
            .as_str()
}
