fn main() {
    let p1 = Character::new(String::from("Pepe"), 100, 50, 25);
    let p2 = Character::new(String::from("Rat"), 20, 30, 10);

    let p1_ = p2.atk(&p1);
    // assert_eq!(p1_.hp, 997);
    println!("{:?}", p1_);

    let p2_ = p1.atk(&p2);
    // assert_eq!(p1_.hp, 0);
    println!("{:?}", p2_);
}

#[derive(Debug)]
struct Character {
    name: String,
    hp: u32,
    atk: u32,
    def: u32,
}

impl Character {
    fn new(name: String, hp: u32, atk: u32, def: u32) -> Character {
        Character { name, hp, atk, def }
    }

    fn atk(&self, other: &Character) -> Character {
        let damage = self.atk * self.atk / (other.def + self.atk);
        let new_hp = if other.hp < damage {
            0
        } else {
            other.hp - damage
        };
        Character::new(other.name.clone(), new_hp, other.atk, other.def)
    }
}
