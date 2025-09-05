pub fn verse(n: u32) -> String {
    if n == 0 {
        String::from(
            "\
No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.",
        )
    } else if n == 1 {
        String::from(
            "\
1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.",
        )
    } else {
        format!(
            "\
{} bottle{} of beer on the wall, {0} bottle{1} of beer.
Take one down and pass it around, {} bottle{} of beer on the wall.",
            n,
            if n == 1 { "" } else { "s" },
            n - 1,
            if n - 1 == 1 { "" } else { "s" },
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    for n in (end..=start).rev() {
        s.push_str(verse(n).as_str());
        s.push_str("\n\n");
    }
    s
}
