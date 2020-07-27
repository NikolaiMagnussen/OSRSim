use serde::{Deserialize};

mod store;

#[allow(dead_code)]
enum StrengthPotion {
    NONE,
    STRENGTH,
    SUPERSTRENGTH,
}

#[allow(dead_code)]
enum AttackPotion {
    NONE,
    ATTACK,
    SUPERATTACK,
}

#[allow(dead_code)]
enum AttackPrayer {
    NONE,
    CLARITY,
    IMPROVED,
    INCREDIBLE,
    CHIVALRY,
    PIETY,
}

#[allow(dead_code)]
enum StrengthPrayer {
    NONE,
    BURST,
    SUPERHUMAN,
    ULTIMATE,
    CHIVALRY,
    PIETY,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum AttackStyle {
    AGGRESSIVE,
    CONTROLLED,
    ACCURATE,
    DEFENSIVE,
}

#[allow(dead_code)]
enum SetBonus {
    VOID,
    NONE,
}

#[allow(dead_code)]
enum HeadSlot {
    SLAYER,
    NONE,
}

#[allow(dead_code)]
enum NeckSlot {
    SALVEREGULAR,
    SALVEENHANCED,
    NONE,
}

struct Gear {
    set_bonus: SetBonus,
    head: HeadSlot,
    neck: NeckSlot,
    weapon: Weapon,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    name: String,
}

impl Gear {
    pub fn new(set_bonus: SetBonus, head: HeadSlot, neck: NeckSlot, weapon: Weapon) -> Self {
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
        self.weapon.attack_interval()
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
            AttackStyle::DEFENSIVE => 0,
        }
    }

    fn strength_prayer_bonus(&self) -> f64 {
        match &self.strength_prayer {
            StrengthPrayer::NONE => 1.0,
            StrengthPrayer::BURST => 1.05,
            StrengthPrayer::SUPERHUMAN => 1.1,
            StrengthPrayer::ULTIMATE => 1.15,
            StrengthPrayer::CHIVALRY => 1.18,
            StrengthPrayer::PIETY => 1.23,
        }
    }

    fn strength_potion_bonus(&self) -> usize {
        let bonus = match &self.strength_potion {
            StrengthPotion::NONE => 0.0,
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
            AttackStyle::DEFENSIVE => 0,
        }
    }

    fn attack_prayer_bonus(&self) -> f64 {
        match &self.attack_prayer {
            AttackPrayer::NONE => 1.0,
            AttackPrayer::CLARITY => 1.05,
            AttackPrayer::IMPROVED => 1.1,
            AttackPrayer::INCREDIBLE => 1.15,
            AttackPrayer::CHIVALRY => 1.15,
            AttackPrayer::PIETY => 1.2,
        }
    }

    fn attack_potion_bonus(&self) -> usize {
        let bonus = match &self.attack_potion {
            AttackPotion::NONE => 0.0,
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

    pub fn max_hit(&self, monster: &Monster, on_task: bool) -> usize {
        let hit = 0.5 + self.effective_strength_level() as f64 * (self.strength_equipment_bonus + 64) as f64 / 640.0;
        let after_bonus = match monster.is_undead() {
            false => hit.floor() * self.gear.regular_bonus(on_task),
            true => hit.floor() * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as usize
    }

    pub fn max_attack_roll(&self, monster: &Monster, on_task: bool) -> usize {
        let roll = self.effective_attack_level() * (self.attack_equipment_bonus + 64);
        let after_bonus = match monster.is_undead() {
            false => roll as f64 * self.gear.regular_bonus(on_task),
            true => roll as f64 * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as usize
    }

    pub fn hit_chance(&self, monster: &Monster, on_task: bool) -> f64 {
        let attack = self.max_attack_roll(monster, on_task) as f64;
        let defence = monster.max_defence_roll(self.gear.weapon.attack_type()) as f64;

        if attack > defence {
            1.0 - (defence + 2.0) / (2.0 * (attack + 1.0))
        } else {
            attack / (2.0 * defence + 1.0)
        }
    }

    pub fn dps(&self, monster: &Monster, on_task: bool) -> f64 {
        self.hit_chance(monster, on_task) * (self.max_hit(monster, on_task) as f64 / 2.0) / self.gear.attack_interval()
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum DefenceStyle {
    STAB,
    SLASH,
    CRUSH,
    MAGIC,
    RANGED,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Monster {
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

    fn max_defence_roll(&self, defence_style: &DefenceStyle) -> usize {
        self.effective_defence_level() * (self.defence_equipment_bonus(defence_style) + 64)
    }

    fn is_undead(&self) -> bool {
        self.attributes.contains(&String::from("undead"))
    }
}

#[derive(Deserialize, Debug, Clone)]
struct WeaponStance {
    combat_style: String,
    attack_type: DefenceStyle,
    attack_style: AttackStyle,
}

#[derive(Deserialize, Debug, Clone)]
struct _Weapon {
    attack_speed: usize,
    stances: Vec<WeaponStance>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weapon {
    name: String,
    weapon: _Weapon,
}

impl Weapon {
    fn attack_interval(&self) -> f64 {
        self.weapon.attack_speed as f64 * 0.6
    }

    fn attack_type(&self) -> &DefenceStyle {
        &self.weapon.stances[0].attack_type
    }
}

/* What modules to have:
 * - main (orchestrate everything - for now)
 * - store (for querying items, via API, parsed file or other way)
 * - player (for handling everything with an instance of a player)
 * - simulation (for generating "player instances" for evaulation and comparing results)
 */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monster_name = "Aberrant spectre";
    let weapon_name = "Abyssal whip";

    let api = store::ApiStore::connect("https://api.osrsbox.com");
    let weapon = api.get_weapon(weapon_name).await?;
    let monster = api.get_monster(monster_name).await?;

    let player = Player::new("Supergeni", 74, 74, AttackPotion::ATTACK, 132, AttackPrayer::NONE,
                             StrengthPotion::STRENGTH, 110, StrengthPrayer::NONE,
                             AttackStyle::CONTROLLED, Gear::new(SetBonus::NONE, HeadSlot::SLAYER,
                                                              NeckSlot::NONE, weapon));

    // Compared to osrs-genie.com - we have the correct DPS, because the author
    // of the tool has not divided the max hit by two to get the average hit
    // resulting in the DPS being double.
    println!("{} has {} DPS against {}", player.name, player.dps(&monster, true), monster.name);
    println!("Getting the shark: {:#?}", api.get_item("Shark").await?);

    Ok(())
}
