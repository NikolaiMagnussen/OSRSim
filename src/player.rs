use serde::Deserialize;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum StrengthPotion {
    NONE,
    STRENGTH,
    SUPERSTRENGTH,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AttackPotion {
    NONE,
    ATTACK,
    SUPERATTACK,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AttackPrayer {
    NONE,
    CLARITY,
    IMPROVED,
    INCREDIBLE,
    CHIVALRY,
    PIETY,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum StrengthPrayer {
    NONE,
    BURST,
    SUPERHUMAN,
    ULTIMATE,
    CHIVALRY,
    PIETY,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AttackStyle {
    AGGRESSIVE,
    CONTROLLED,
    ACCURATE,
    DEFENSIVE,
    RANGED,
    MAGIC,
}

impl fmt::Display for AttackStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AttackStyle::MAGIC => "magic",
                AttackStyle::RANGED => "ranged",
                AttackStyle::DEFENSIVE => "defensive",
                AttackStyle::ACCURATE => "accurate",
                AttackStyle::CONTROLLED => "controlled",
                AttackStyle::AGGRESSIVE => "agressive",
            }
        )
    }
}

impl fmt::Display for AttackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AttackType::RANGED => "ranged",
                AttackType::MAGIC => "magic",
                AttackType::STAB => "stab",
                AttackType::CRUSH => "crush",
                AttackType::SLASH => "slash",
                AttackType::SPELLCASTING => "spellcasting",
                AttackType::DEFENSIVECASTING => "defensive casting",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gear {
    pub weapon: Option<Weapon>,
    pub equipment: HashMap<EquipmentSlot, Option<Equipment>>,
}

#[derive(Debug, Clone)]
pub struct SpareGear {
    pub spare_weapons: Vec<Weapon>,
    pub spare_equipment: Vec<Equipment>,
}

impl SpareGear {
    pub fn new() -> Self {
        SpareGear {
            spare_weapons: Vec::new(),
            spare_equipment: Vec::new(),
        }
    }

    pub fn add_weapon(&mut self, weapon: Option<&Weapon>) {
        if let Some(weapon) = weapon {
            self.spare_weapons.push(weapon.clone());
        }
    }

    pub fn add_equipment(&mut self, equipment: Option<&Equipment>) {
        if let Some(equipment) = equipment {
            self.spare_equipment.push(equipment.clone());
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentSlot {
    RING,
    FEET,
    HANDS,
    NECK,
    AMMO,
    CAPE,
    BODY,
    LEGS,
    HEAD,
    SHIELD,
    WEAPON,
    #[serde(rename = "2h")]
    TWOHAND,
}

impl fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EquipmentSlot::RING => "ring",
                EquipmentSlot::FEET => "feet",
                EquipmentSlot::HANDS => "hands",
                EquipmentSlot::NECK => "neck",
                EquipmentSlot::AMMO => "ammo",
                EquipmentSlot::CAPE => "cape",
                EquipmentSlot::BODY => "body",
                EquipmentSlot::LEGS => "legs",
                EquipmentSlot::HEAD => "head",
                EquipmentSlot::SHIELD => "shield",
                EquipmentSlot::WEAPON => "wep",
                EquipmentSlot::TWOHAND => "2h",
            }
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum WeaponSlot {
    TWOHAND(Weapon),
    ONEHAND(Weapon, Equipment),
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct _Equipment {
    attack_stab: isize,
    attack_slash: isize,
    attack_crush: isize,
    attack_magic: isize,
    attack_ranged: isize,
    defence_stab: isize,
    defence_slash: isize,
    defence_crush: isize,
    defence_magic: isize,
    defence_ranged: isize,
    melee_strength: isize,
    ranged_strength: isize,
    magic_damage: isize,
    prayer: isize,
    pub slot: EquipmentSlot,
}

impl _Equipment {
    pub fn unarmed() -> Self {
        _Equipment {
            attack_stab: 0,
            attack_slash: 0,
            attack_crush: 0,
            attack_magic: 0,
            attack_ranged: 0,
            defence_stab: 0,
            defence_slash: 0,
            defence_crush: 0,
            defence_magic: 0,
            defence_ranged: 0,
            melee_strength: 0,
            ranged_strength: 0,
            magic_damage: 0,
            prayer: 0,
            slot: EquipmentSlot::TWOHAND,
        }
    }

    pub fn attack_bonus(&self, style: &AttackType) -> isize {
        match style {
            AttackType::STAB => self.attack_stab,
            AttackType::SLASH => self.attack_slash,
            AttackType::CRUSH => self.attack_crush,
            AttackType::RANGED => self.attack_ranged,
            AttackType::MAGIC | AttackType::SPELLCASTING | AttackType::DEFENSIVECASTING => {
                self.attack_magic
            }
        }
    }

    pub fn strength_bonus(&self, style: &AttackType) -> isize {
        match style {
            AttackType::STAB | AttackType::SLASH | AttackType::CRUSH => self.melee_strength,
            AttackType::RANGED => self.ranged_strength,
            AttackType::MAGIC | AttackType::SPELLCASTING | AttackType::DEFENSIVECASTING => {
                self.magic_damage
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equipment {
    pub name: String,
    pub equipment: _Equipment,
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment {
            name: String::new(),
            equipment: _Equipment::unarmed(),
        }
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name.is_empty() {
            write!(f, "NONE")
        } else {
            write!(f, "({}) {}", self.equipment.slot, self.name)
        }
    }
}

impl Gear {
    pub fn empty() -> Self {
        Gear {
            weapon: None,
            equipment: HashMap::new(),
        }
    }

    pub fn add_equipment(&mut self, slot: &EquipmentSlot, equipment: Option<Equipment>) {
        self.equipment.insert(*slot, equipment.clone());
    }

    pub fn add_weapon(&mut self, weapon: Option<Weapon>) {
        self.weapon = weapon.clone();
    }

    pub fn void_bonus(&self) -> f64 {
        let head = self.equipment.get(&EquipmentSlot::HEAD);
        let body = self.equipment.get(&EquipmentSlot::BODY);
        let legs = self.equipment.get(&EquipmentSlot::LEGS);
        let hands = self.equipment.get(&EquipmentSlot::HANDS);
        match (head, body, legs, hands) {
            (Some(Some(head)), Some(Some(body)), Some(Some(legs)), Some(Some(hands)))
                if (head.name == "Void melee helm"
                    || head.name == "Void ranger helm"
                    || head.name == "Void mage helm")
                    && (body.name == "Void knight top" || body.name == "Elite void top")
                    && (legs.name == "Void knight robe" || legs.name == "Elite void robe")
                    && hands.name == "Void knight gloves" =>
            {
                1.1
            }
            _ => 1.0,
        }
    }

    pub fn regular_bonus(&self, on_task: bool) -> f64 {
        match self.equipment.get(&EquipmentSlot::HEAD) {
            Some(Some(head))
                if (head.name == "Slayer helmet" || head.name == "Slayer helmet (i)")
                    && on_task =>
            {
                7.0 / 6.0
            }
            _ => 1.0,
        }
    }

    pub fn undead_bonus(&self, on_task: bool) -> f64 {
        match self.equipment.get(&EquipmentSlot::NECK) {
            Some(Some(neck)) if neck.name == "Salve amulet" || neck.name == "Salve amulet(i)" => {
                7.0 / 6.0
            }
            Some(Some(neck))
                if neck.name == "Salve amulet (e)" || neck.name == "Salve amulet(ei)" =>
            {
                1.2
            }
            _ => self.regular_bonus(on_task),
        }
    }

    pub fn attack_equipment_bonus(&self, style: &AttackType) -> isize {
        let bonus: isize = self
            .equipment
            .values()
            .map(|y| y.as_ref().map_or(0, |x| x.equipment.attack_bonus(style)))
            .sum();
        bonus
            + self
                .weapon
                .as_ref()
                .map_or(0, |x| x.equipment.attack_bonus(style))
    }

    pub fn strength_equipment_bonus(&self, style: &AttackType) -> isize {
        let bonus: isize = self
            .equipment
            .values()
            .map(|y| y.as_ref().map_or(0, |x| x.equipment.strength_bonus(style)))
            .sum();
        bonus
            + self
                .weapon
                .as_ref()
                .map_or(0, |x| x.equipment.strength_bonus(style))
    }

    pub fn attack_interval(&self) -> f64 {
        self.weapon
            .as_ref()
            .map_or(4.0 * 0.6, |x| x.attack_interval())
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    attack: isize,
    strength: isize,
    attack_potion: AttackPotion,
    attack_prayer: AttackPrayer,
    strength_potion: StrengthPotion,
    strength_prayer: StrengthPrayer,
    pub gear: Gear,
    pub spare_equipment: SpareGear,
}

impl Player {
    pub fn new(
        name: &str,
        attack: isize,
        strength: isize,
        attack_potion: AttackPotion,
        attack_prayer: AttackPrayer,
        strength_potion: StrengthPotion,
        strength_prayer: StrengthPrayer,
        gear: Gear,
    ) -> Self {
        Player {
            name: String::from(name),
            attack: attack,
            strength: strength,
            attack_potion: attack_potion,
            attack_prayer: attack_prayer,
            strength_potion: strength_potion,
            strength_prayer: strength_prayer,
            gear: gear,
            spare_equipment: SpareGear::new(),
        }
    }

    fn strength_style_bonus(&self, attack_style: &AttackStyle) -> isize {
        match attack_style {
            AttackStyle::ACCURATE => 0,
            AttackStyle::AGGRESSIVE => 3,
            AttackStyle::CONTROLLED => 1,
            AttackStyle::DEFENSIVE => 0,
            _ => 0,
        }
    }

    pub fn weapon_styles(&self) -> Vec<(AttackStyle, AttackType)> {
        self.gear
            .weapon
            .as_ref()
            .map_or(_Weapon::default(), |x| x.weapon.clone())
            .stances
            .iter()
            .map(|x| (x.attack_style.unwrap(), x.attack_type.unwrap()))
            .collect()
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

    fn strength_potion_bonus(&self) -> isize {
        let bonus = match &self.strength_potion {
            StrengthPotion::NONE => 0.0,
            StrengthPotion::STRENGTH => self.strength as f64 * 0.1 + 3.0,
            StrengthPotion::SUPERSTRENGTH => self.strength as f64 * 0.15 + 5.0,
        };
        bonus.floor() as isize
    }

    fn attack_style_bonus(&self, attack_style: &AttackStyle) -> isize {
        match attack_style {
            AttackStyle::ACCURATE => 3,
            AttackStyle::AGGRESSIVE => 0,
            AttackStyle::CONTROLLED => 1,
            AttackStyle::DEFENSIVE => 0,
            _ => 0,
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

    fn attack_potion_bonus(&self) -> isize {
        let bonus = match &self.attack_potion {
            AttackPotion::NONE => 0.0,
            AttackPotion::ATTACK => self.attack as f64 * 0.1 + 3.0,
            AttackPotion::SUPERATTACK => self.attack as f64 * 0.15 + 5.0,
        };
        bonus.floor() as isize
    }

    fn effective_strength_level(&self, attack_style: &AttackStyle) -> isize {
        let potion = self.strength + self.strength_potion_bonus();
        let prayer = potion as f64 * self.strength_prayer_bonus();
        let style = prayer.floor() as isize + self.strength_style_bonus(attack_style) + 8;
        let bonus = style as f64 * self.gear.void_bonus();
        bonus.floor() as isize
    }

    fn effective_attack_level(&self, attack_style: &AttackStyle) -> isize {
        let potion = self.attack + self.attack_potion_bonus();
        let prayer = potion as f64 * self.attack_prayer_bonus();
        let style = prayer.floor() as isize + self.attack_style_bonus(attack_style) + 8;
        let bonus = style as f64 * self.gear.void_bonus();
        bonus.floor() as isize
    }

    pub fn max_hit(
        &self,
        monster: &Monster,
        on_task: bool,
        attack_style: &AttackStyle,
        attack_type: &AttackType,
    ) -> isize {
        let hit = 0.5
            + self.effective_strength_level(attack_style) as f64
                * (self.gear.strength_equipment_bonus(&attack_type) + 64) as f64
                / 640.0;
        let after_bonus = match monster.is_undead() {
            false => hit.floor() * self.gear.regular_bonus(on_task),
            true => hit.floor() * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as isize
    }

    pub fn max_attack_roll(
        &self,
        monster: &Monster,
        on_task: bool,
        attack_style: &AttackStyle,
        attack_type: &AttackType,
    ) -> isize {
        let roll = self.effective_attack_level(attack_style)
            * (self.gear.attack_equipment_bonus(attack_type) + 64);
        let after_bonus = match monster.is_undead() {
            false => roll as f64 * self.gear.regular_bonus(on_task),
            true => roll as f64 * self.gear.undead_bonus(on_task),
        };
        after_bonus.floor() as isize
    }

    pub fn hit_chance(
        &self,
        monster: &Monster,
        on_task: bool,
        style: &(AttackStyle, AttackType),
    ) -> f64 {
        let attack = self.max_attack_roll(monster, on_task, &style.0, &style.1) as f64;
        let defence = monster.max_defence_roll(&style.1) as f64;

        if attack > defence {
            1.0 - (defence + 2.0) / (2.0 * (attack + 1.0))
        } else {
            attack / (2.0 * defence + 1.0)
        }
    }

    pub fn dps(&self, monster: &Monster, on_task: bool, style: &(AttackStyle, AttackType)) -> f64 {
        self.hit_chance(monster, on_task, style)
            * (self.max_hit(monster, on_task, &style.0, &style.1) as f64 / 2.0)
            / self.gear.attack_interval()
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AttackType {
    STAB,
    SLASH,
    CRUSH,
    RANGED,
    MAGIC,
    SPELLCASTING,
    #[serde(rename = "defensive casting")]
    DEFENSIVECASTING,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Monster {
    pub name: String,
    defence_level: isize,
    defence_stab: isize,
    defence_slash: isize,
    defence_crush: isize,
    defence_magic: isize,
    defence_ranged: isize,
    attributes: Vec<String>,
}

impl Monster {
    fn effective_defence_level(&self) -> isize {
        self.defence_level + 1 + 8
    }

    fn defence_equipment_bonus(&self, attack_type: &AttackType) -> isize {
        match attack_type {
            AttackType::STAB => self.defence_stab,
            AttackType::SLASH => self.defence_slash,
            AttackType::CRUSH => self.defence_crush,
            AttackType::RANGED => self.defence_ranged,
            AttackType::SPELLCASTING | AttackType::DEFENSIVECASTING | AttackType::MAGIC => {
                self.defence_magic
            }
        }
    }

    fn max_defence_roll(&self, attack_type: &AttackType) -> isize {
        self.effective_defence_level() * (self.defence_equipment_bonus(attack_type) + 64)
    }

    fn is_undead(&self) -> bool {
        self.attributes.contains(&String::from("undead"))
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WeaponStance {
    combat_style: String,
    pub attack_type: Option<AttackType>,
    pub attack_style: Option<AttackStyle>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct _Weapon {
    pub attack_speed: isize,
    pub stances: Vec<WeaponStance>,
}

impl Default for _Weapon {
    fn default() -> Self {
        _Weapon {
            attack_speed: 4,
            stances: vec![
                WeaponStance {
                    combat_style: String::from("kick"),
                    attack_type: Some(AttackType::CRUSH),
                    attack_style: Some(AttackStyle::AGGRESSIVE),
                },
                WeaponStance {
                    combat_style: String::from("punch"),
                    attack_type: Some(AttackType::CRUSH),
                    attack_style: Some(AttackStyle::ACCURATE),
                },
                WeaponStance {
                    combat_style: String::from("block"),
                    attack_type: Some(AttackType::CRUSH),
                    attack_style: Some(AttackStyle::DEFENSIVE),
                },
            ],
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Weapon {
    pub name: String,
    pub weapon: _Weapon,
    pub equipment: _Equipment,
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) {}", self.equipment.slot, self.name)
    }
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            name: String::from("Unarmed"),
            weapon: _Weapon::default(),
            equipment: _Equipment::unarmed(),
        }
    }
}

impl Weapon {
    fn attack_interval(&self) -> f64 {
        self.weapon.attack_speed as f64 * 0.6
    }

    fn attack_type(&self, attack_style: usize) -> &AttackType {
        &self.weapon.stances[attack_style]
            .attack_type
            .as_ref()
            .unwrap()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
}
*/
