use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HighScore {
    score: u64,
    high_score: u64,
}

const SCORE_FILE: &str = "highscore.dat";

impl HighScore {
    pub fn load() -> Self {
        let high_score = fs::read_to_string(SCORE_FILE)
            .map_or(Ok(0), |i| i.parse::<u64>())
            .unwrap_or(0);

        HighScore {
            score: 0,
            high_score,
        }
    }

    pub fn current_score(&self) -> u64 {
        self.score
    }

    pub fn high_score(&self) -> u64 {
        self.high_score
    }

    pub fn set_high_score(&mut self, high_score: u64) {
        self.high_score = high_score;
    }

    pub fn add(&mut self, addition: u64) {
        self.score += addition;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn save(&mut self) {
        _ = fs::write(SCORE_FILE, self.high_score.to_string());
    }
}
