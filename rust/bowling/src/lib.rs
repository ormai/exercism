#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    throws: Vec<u16>,
    knocked: u16, // pins knocked in the first throw of this frame
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            throws: Vec::with_capacity(21),
            knocked: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.knocked + pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.throws.push(pins);
            self.knocked = if self.knocked + pins == 10 { 0 } else { pins };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut score = 0;
        let mut stage = 0;
        for _ in 0..10 {
            let first = self.throws.get(stage)?;
            let second = self.throws.get(stage + 1)?;
            score += first + second;
            if first + second >= 10 {
                score += self.throws.get(stage + 2)?;
            }
            stage += if *first == 10 { 1 } else { 2 };
        }
        Some(score)
    }
}
