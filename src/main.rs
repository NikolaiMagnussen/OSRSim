use serde::{Deserialize};
use std::io::{Error, ErrorKind};

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

struct WeaponSlot {
    name: String,
    ticks: f64,
    defence_style: DefenceStyle,
}

impl WeaponSlot {
    pub fn new(name: &str, ticks: f64, defence_style: DefenceStyle) -> Self {
        WeaponSlot {
            name: String::from(name),
            ticks: ticks,
            defence_style: defence_style,
        }
    }
}

struct Gear {
    set_bonus: SetBonus,
    head: HeadSlot,
    neck: NeckSlot,
    weapon: WeaponSlot,
}

impl Gear {
    pub fn new(set_bonus: SetBonus, head: HeadSlot, neck: NeckSlot, weapon: WeaponSlot) -> Self {
        Gear {
            set_bonus: set_bonus,
            head: head,
            neck: neck,
            weapon: weapon,
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
        self.weapon.ticks * 0.6
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
}

impl MonsterStuff for Enemy {
    fn max_defence_roll(&self, _defence_style: &DefenceStyle) -> usize {
        self.effective_defence_level() * (self.defence_equipment_bonus + 64)
    }

    fn is_undead(&self) -> bool {
        self.kind == MonsterType::UNDEAD
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

    pub fn max_hit(&self, monster: &impl MonsterStuff, on_task: bool) -> usize {
        let hit = 0.5 + self.effective_strength_level() as f64 * (self.strength_equipment_bonus + 64) as f64 / 640.0;
        let after_bonus = match monster.is_undead() {
            false => hit.floor() * self.gear.regular_bonus(on_task),
            true => hit.floor() * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as usize
    }

    pub fn max_attack_roll(&self, monster: &impl MonsterStuff, on_task: bool) -> usize {
        let roll = self.effective_attack_level() * (self.attack_equipment_bonus + 64);
        let after_bonus = match monster.is_undead() {
            false => roll as f64 * self.gear.regular_bonus(on_task),
            true => roll as f64 * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as usize
    }

    pub fn hit_chance(&self, monster: &impl MonsterStuff, on_task: bool) -> f64 {
        let attack = self.max_attack_roll(monster, on_task) as f64;
        let defence = monster.max_defence_roll(&self.gear.weapon.defence_style) as f64;

        if attack > defence {
            1.0 - (defence + 2.0) / (2.0 * (attack + 1.0))
        } else {
            attack / (2.0 * defence + 1.0)
        }
    }

    pub fn dps(&self, monster: &impl MonsterStuff, on_task: bool) -> f64 {
        self.hit_chance(monster, on_task) * (self.max_hit(monster, on_task) as f64 / 2.0) / self.gear.attack_interval()
    }
}

trait MonsterStuff {
    fn max_defence_roll(&self, defence_style: &DefenceStyle) -> usize;
    fn is_undead(&self) -> bool;
}

enum DefenceStyle {
    STAB,
    SLASH,
    CRUSH,
    MAGIC,
    RANGED,
}

#[derive(Deserialize, Debug, Clone)]
struct Monster {
    name: String,
    defence_level: usize,
    defence_stab: usize,
    defence_slash: usize,
    defence_crush: usize,
    defence_magic: usize,
    defence_ranged: usize,
    attributes: Vec<String>,
}

impl Monster {
    fn effective_defence_level(&self) -> usize {
        self.defence_level + 1 + 8
    }

    fn defence_equipment_bonus(&self, defence_style: &DefenceStyle) -> usize {
        match defence_style {
            DefenceStyle::STAB => self.defence_stab,
            DefenceStyle::SLASH => self.defence_slash,
            DefenceStyle::CRUSH => self.defence_crush,
            DefenceStyle::MAGIC => self.defence_magic,
            DefenceStyle::RANGED => self.defence_ranged,
        }
    }
}

impl MonsterStuff for Monster {
    fn max_defence_roll(&self, defence_style: &DefenceStyle) -> usize {
        self.effective_defence_level() * (self.defence_equipment_bonus(defence_style) + 64)
    }

    fn is_undead(&self) -> bool {
        self.attributes.contains(&String::from("undead"))
    }
}


#[derive(Deserialize, Debug)]
struct Monsters<T> {
    _items: Vec<T>,
}

async fn get_monster(name: &str) -> Result<Monster, Box<dyn std::error::Error>> {
    let monsters = reqwest::get(&format!(r#"https://api.osrsbox.com/monsters?where={{"name":"{}","duplicate": false }}"#, name))
        .await?
        .json::<Monsters<Monster>>()
        .await?;
    if monsters._items.len() > 0 {
        Ok(monsters._items[0].clone())
    } else {
        Err(Box::new(Error::new(ErrorKind::InvalidData, "The monster does not exist..")))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player = Player::new("Supergeni", 97, 99, AttackPotion::SUPERATTACK, 136, AttackPrayer::PIETY,
                             StrengthPotion::SUPERSTRENGTH, 133, StrengthPrayer::PIETY,
                             AttackStyle::ACCURATE, Gear::new(SetBonus::NONE, HeadSlot::SLAYER,
                                                              NeckSlot::NONE, WeaponSlot::new("Abyssal whip", 4.0, DefenceStyle::SLASH)));
    let abby = get_monster("Mithril dragon").await?;
    println!("{} has {} DPS against {}", player.name, player.dps(&abby, true), abby.name);
    Ok(())
}
