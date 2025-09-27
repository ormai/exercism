pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle = vec![Vec::new(); self.0 as usize];
        if self.0 > 0 {
            triangle[0] = vec![1];
        }
        for i in 1..triangle.len() {
            let len = triangle[i - 1].len() + 1;
            triangle[i].resize(len, 1);
            for j in 0..len {
                if (1..len - 1).contains(&j) {
                    triangle[i][j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
                }
            }
        }
        triangle
    }
}
