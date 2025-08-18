use rand::{Rng, rng};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub life: u32,
}

impl Enemy {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            life: rng().random_range(75..200),
        }
    }
}

impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enemy: {} life: {}", self.name, self.life,)
    }
}
