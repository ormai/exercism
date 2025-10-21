pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = Vec::with_capacity(4);
    for i in 0..4 {
        if n & 1 << i != 0 {
            actions.push(["wink", "double blink", "close your eyes", "jump"][i]);
        }
    }
    if n & 16 == 16 {
        actions.reverse();
    }
    actions
}
