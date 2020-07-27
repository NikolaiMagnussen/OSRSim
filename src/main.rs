enum StrengthPotion {
    STRENGTH,
    SUPERSTRENGTH,
}

enum AttackPotion {
    ATTACK,
    SUPERATTACK,
}

enum AttackPrayer {
    CLARITY,
    IMPROVED,
    INCREDIBLE,
    CHIVALRY,
    PIETY,
}

enum StrengthPrayer {
    BURST,
    SUPERHUMAN,
    ULTIMATE,
    CHIVALRY,
    PIETY,
}

enum AttackStyle {
    AGGRESSIVE,
    CONTROLLED,
    ACCURATE,
}

enum SetBonus {
    VOID,
    NONE,
}

enum HeadSlot {
    SLAYER,
    NONE,
}

enum NeckSlot {
    SALVEREGULAR,
    SALVEENHANCED,
    NONE,
}

#[derive(PartialEq)]
enum MonsterType {
    REGULAR,
    UNDEAD,
}

struct Gear {
    set_bonus: SetBonus,
    head: HeadSlot,
    neck: NeckSlot,
    weapon_ticks: f64,
}

impl Gear {
    pub fn new(set_bonus: SetBonus, head: HeadSlot, neck: NeckSlot, weapon_ticks: f64) -> Self {
        Gear {
            set_bonus: set_bonus,
            head: head,
            neck: neck,
            weapon_ticks: weapon_ticks,
        }
    }

    pub fn void_bonus(&self) -> f64 {
        match &self.set_bonus {
            SetBonus::NONE => 1.0,
            SetBonus::VOID => 1.1,
        }
    }

    pub fn regular_bonus(&self, on_task: bool) -> f64 {
        match &self.head {
            HeadSlot::SLAYER if on_task => 7.0/6.0,
            _ => 1.0,
        }
    }

    pub fn undead_bonus(&self, on_task: bool) -> f64 {
        match &self.neck {
            NeckSlot::SALVEENHANCED => 1.2,
            NeckSlot::SALVEREGULAR => 7.0/6.0,
            NeckSlot::NONE => self.regular_bonus(on_task),
        }
    }

    pub fn attack_interval(&self) -> f64 {
        self.weapon_ticks * 0.6
    }
}

struct Enemy {
    name: String,
    defence: usize,
    defence_equipment_bonus: usize,
    kind: MonsterType,
}

impl Enemy {
    pub fn new(name: &str, defence: usize, defence_equipment_bonus: usize, kind: MonsterType) -> Self {
        Enemy {
            name: String::from(name),
            defence: defence,
            defence_equipment_bonus: defence_equipment_bonus,
            kind: kind,
        }
    }

    fn effective_defence_level(&self) -> usize {
        self.defence + 1 + 8
    }

    fn is_undead(&self) -> bool {
        self.kind == MonsterType::UNDEAD
    }

    pub fn max_defence_roll(&self) -> usize {
        self.effective_defence_level() * (self.defence_equipment_bonus + 64)
    }
}

struct Player {
    name: String,
    attack: usize,
    strength: usize,
    attack_potion: AttackPotion,
    attack_prayer: AttackPrayer,
    attack_equipment_bonus: usize,
    strength_potion: StrengthPotion,
    strength_prayer: StrengthPrayer,
    strength_equipment_bonus: usize,
    attack_style: AttackStyle,
    gear: Gear,
}

impl Player {
    pub fn new(name: &str, attack: usize, strength: usize, attack_potion: AttackPotion,
               attack_equipment_bonus: usize, attack_prayer: AttackPrayer, strength_potion: StrengthPotion,
               strength_equipment_bonus: usize, strength_prayer: StrengthPrayer, attack_style: AttackStyle, gear: Gear) -> Self {
        Player {
            name: String::from(name),
            attack: attack,
            strength: strength,
            attack_potion: attack_potion,
            attack_equipment_bonus: attack_equipment_bonus,
            attack_prayer: attack_prayer,
            strength_potion: strength_potion,
            strength_equipment_bonus: strength_equipment_bonus,
            strength_prayer: strength_prayer,
            attack_style: attack_style,
            gear: gear,
        }
    }

    fn strength_style_bonus(&self) -> usize {
        match &self.attack_style {
            AttackStyle::ACCURATE => 0,
            AttackStyle::AGGRESSIVE => 3,
            AttackStyle::CONTROLLED => 1,
        }
    }

    fn strength_prayer_bonus(&self) -> f64 {
        match &self.strength_prayer {
            StrengthPrayer::BURST => 1.05,
            StrengthPrayer::SUPERHUMAN => 1.1,
            StrengthPrayer::ULTIMATE => 1.15,
            StrengthPrayer::CHIVALRY => 1.18,
            StrengthPrayer::PIETY => 1.23,
        }
    }

    fn strength_potion_bonus(&self) -> usize {
        let bonus = match &self.strength_potion {
            StrengthPotion::STRENGTH => self.strength as f64 * 0.1 + 3.0,
            StrengthPotion::SUPERSTRENGTH => self.strength as f64 * 0.15 + 5.0,
        };
        bonus.floor() as usize
    }

    fn attack_style_bonus(&self) -> usize {
        match &self.attack_style {
            AttackStyle::ACCURATE => 3,
            AttackStyle::AGGRESSIVE => 0,
            AttackStyle::CONTROLLED => 1,
        }
    }

    fn attack_prayer_bonus(&self) -> f64 {
        match &self.attack_prayer {
            AttackPrayer::CLARITY => 1.05,
            AttackPrayer::IMPROVED => 1.1,
            AttackPrayer::INCREDIBLE => 1.15,
            AttackPrayer::CHIVALRY => 1.15,
            AttackPrayer::PIETY => 1.2,
        }
    }

    fn attack_potion_bonus(&self) -> usize {
        let bonus = match &self.attack_potion {
            AttackPotion::ATTACK => self.attack as f64 * 0.1 + 3.0,
            AttackPotion::SUPERATTACK => self.attack as f64 * 0.15 + 5.0,
        };
        bonus.floor() as usize
    }

    fn effective_strength_level(&self) -> usize {
        let potion = self.strength + self.strength_potion_bonus();
        let prayer = potion as f64 * self.strength_prayer_bonus();
        let style = prayer.floor() as usize + self.strength_style_bonus() + 8;
        let bonus = style as f64 * self.gear.void_bonus();
        bonus.floor() as usize
    }

    fn effective_attack_level(&self) -> usize {
        let potion = self.attack + self.attack_potion_bonus();
        let prayer = potion as f64 * self.attack_prayer_bonus();
        let style = prayer.floor() as usize + self.attack_style_bonus() + 8;
        let bonus = style as f64 * self.gear.void_bonus();
        bonus.floor() as usize
    }

    pub fn max_hit(&self, monster: &Enemy, on_task: bool) -> usize {
        let hit = 0.5 + self.effective_strength_level() as f64 * (self.strength_equipment_bonus + 64) as f64 / 640.0;
        let after_bonus = match monster.is_undead() {
            false => hit.floor() * self.gear.regular_bonus(on_task),
            true => hit.floor() * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as usize
    }

    pub fn max_attack_roll(&self, monster: &Enemy, on_task: bool) -> usize {
        let roll = self.effective_attack_level() * (self.attack_equipment_bonus + 64);
        let after_bonus = match monster.is_undead() {
            false => roll as f64 * self.gear.regular_bonus(on_task),
            true => roll as f64 * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as usize
    }

    pub fn hit_chance(&self, monster: &Enemy, on_task: bool) -> f64 {
        let attack = self.max_attack_roll(monster, on_task) as f64;
        let defence = monster.max_defence_roll() as f64;

        if attack > defence {
            1.0 - (defence + 2.0) / (2.0 * (attack + 1.0))
        } else {
            attack / (2.0 * defence + 1.0)
        }
    }

    pub fn dps(&self, monster: &Enemy, on_task: bool) -> f64 {
        self.hit_chance(monster, on_task) * (self.max_hit(monster, on_task) as f64 / 2.0) / self.gear.attack_interval()
    }
}

fn main() {
    let player = Player::new("Supergeni", 97, 99, AttackPotion::SUPERATTACK, 136, AttackPrayer::PIETY,
                             StrengthPotion::SUPERSTRENGTH, 133, StrengthPrayer::PIETY,
                             AttackStyle::ACCURATE, Gear::new(SetBonus::NONE, HeadSlot::SLAYER, NeckSlot::NONE, 4.0));
    let abyss = Enemy::new("Abyssal Demon", 135, 20, MonsterType::REGULAR);

    println!("{} can hit: {}", player.name, player.max_hit(&abyss, true));
    println!("Max attack roll: {}", player.max_attack_roll(&abyss, true));

    println!("{} has max defence roll: {}", abyss.name, abyss.max_defence_roll());

    println!("{} has a hit chance of {} against {}", player.name, player.hit_chance(&abyss, true), abyss.name);
    println!("{} has {} DPS against {}", player.name, player.dps(&abyss, true), abyss.name);
}
