const NUMBERS: [&str; 10] = [
    "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let (start_bottles, take_down) = (start_bottles as usize, take_down as usize);
    let mut song = String::new();
    for bottles in ((start_bottles - take_down + 1)..=start_bottles).rev() {
        if bottles != start_bottles {
            song.push_str("\n\n");
        }
        song.push_str(
            format!(
                "{} green bottle{} hanging on the wall,
{0} green bottle{1} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {} hanging on the wall.",
                NUMBERS[bottles - 1],
                if bottles > 1 { "s" } else { "" },
                if bottles as isize - 1 > 0 {
                    format!(
                        "{} green bottle{}",
                        NUMBERS[(bottles as isize - 1) as usize - 1].to_lowercase(),
                        if bottles as isize - 1 > 1 {
                            "s"
                        } else {
                            ""
                        }
                    )
                } else {
                    "no green bottles".to_string()
                }
            )
            .as_str(),
        );
    }
    song
}
