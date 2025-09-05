pub fn raindrops(n: u32) -> String {
    let mut drops = String::new();

    if n % 3 == 0 {
        drops += "Pling";
    }
    if n % 5 == 0 {
        drops += "Plang";
    }
    if n % 7 == 0 {
        drops += "Plong";
    }

    if drops.is_empty() {
        n.to_string()
    } else {
        drops
    }
}
