struct Karlach {
    name: String,
    health: u8,
}

impl Karlach {
    fn take_damage(&mut self, damage: u8){
        // self.health -= damage;
        // Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing
        self.health = self.health.saturating_sub(damage);
        println!(
            "{} takes {} damage, health points now at {}",
            self.name, damage, self.health
        );
    }

    fn healing(&mut self, healing: u8) {
        if self.health + healing >= 100 {
            self.health = 100;
            return;
        }
        self.health += healing;
        
        println!(
            "{} takes {} healing, health points now at {}",
            self.name, healing, self.health
        );
    }
}

fn main() {
    let mut karlach = Karlach {
        name: String::from("Karlach"),
        health: 100,
    };

    karlach.take_damage(60);
    karlach.healing(5);
}
