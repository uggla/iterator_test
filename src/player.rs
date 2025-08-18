use crate::Poisoned;

#[derive(Debug, Clone)]
pub struct Player {
    score: u32,
    pub name: String,
    poisoned: bool,
    pub life: u32,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player: {} {}: {} life: {}",
            self.name,
            if self.poisoned { "is poisoned" } else { "" },
            self.score,
            self.life,
        )
    }
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            score: 0,
            name: name.to_string(),
            poisoned: false,
            life: 100,
        }
    }

    pub fn add_score(&mut self) {
        if !self.poisoned {
            self.score += 1;
        }
    }
}

impl Poisoned for Player {
    fn poisoned(&mut self) {
        self.poisoned = true;
    }

    fn is_poisoned(&self) -> bool {
        self.poisoned
    }
}
