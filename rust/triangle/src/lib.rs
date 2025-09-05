use std::collections::HashSet;

pub struct Triangle {
    sides: usize,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let [a, b, c] = sides;
        if a != 0 && a + b >= c && b + c >= a && a + c >= b {
            Some(Self {
                sides: HashSet::from(sides).len(),
            })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides <= 2
    }
}
