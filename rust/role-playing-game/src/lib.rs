pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(level: u32) -> Self {
        Self {
            health: 100,
            mana: (level >= 10).then(|| 100),
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        if self.health == 0u32 || self.level >= 10u32 {
            Some(Player::new(self.level))
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana_cost > mana => 0,
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                2 * mana_cost
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}
