#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let capacity: usize = max_weight.try_into().expect("u32 fits into usize");
    let mut table = vec![vec![0u32; capacity + 1]; items.len() + 1];
    for i in 1..=items.len() {
        let w: usize = items[i - 1].weight.try_into().expect("u32 fits into usize");
        for j in 1..=capacity {
            table[i][j] = if w > j {
                table[i - 1][j]
            } else {
                table[i - 1][j].max(table[i - 1][j - w] + items[i - 1].value)
            }
        }
    }
    table[items.len()][capacity]
}
