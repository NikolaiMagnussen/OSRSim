use serde::Deserialize;
use std::collections::HashMap;
use std::cmp::{Eq, PartialEq};
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

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AttackStyle {
    AGGRESSIVE,
    CONTROLLED,
    ACCURATE,
    DEFENSIVE,
    RANGED,
    MAGIC,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SetBonus {
    VOID,
    NONE,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum HeadSlot {
    SLAYER,
    NONE,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NeckSlot {
    SALVEREGULAR,
    SALVEENHANCED,
    NONE,
}

#[derive(Debug, Clone)]
pub struct Gear {
    set_bonus: SetBonus,
    head: HeadSlot,
    neck: NeckSlot,
    pub weapon: Weapon,
    equipment: HashMap<EquipmentSlot, Equipment>,
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
}

#[derive(Deserialize, Debug, Clone)]
pub enum WeaponSlot {
    TWOHAND(Weapon),
    ONEHAND(Weapon, Equipment),
}

#[derive(Deserialize, Debug, Clone)]
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
    slot: EquipmentSlot,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Equipment {
    name: String,
    equipment: _Equipment,
}

impl Gear {
    pub fn new(set_bonus: SetBonus, head: HeadSlot, neck: NeckSlot, weapon: Weapon) -> Self {
        Gear {
            set_bonus: set_bonus,
            head: head,
            neck: neck,
            weapon: weapon,
            equipment: HashMap::new(),
        }
    }

    pub fn add_equipment(&mut self, equipment: Option<&Equipment>) {
        if let Some(equipment) = equipment {
            self.equipment.insert(equipment.equipment.slot, equipment.clone());
        }
    }

    pub fn add_weapon(&mut self, weapon: Option<&Weapon>) {
        if let Some(weapon) = weapon {
            self.weapon = weapon.clone();
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
            HeadSlot::SLAYER if on_task => 7.0 / 6.0,
            _ => 1.0,
        }
    }

    pub fn undead_bonus(&self, on_task: bool) -> f64 {
        match &self.neck {
            NeckSlot::SALVEENHANCED => 1.2,
            NeckSlot::SALVEREGULAR => 7.0 / 6.0,
            NeckSlot::NONE => self.regular_bonus(on_task),
        }
    }

    pub fn attack_interval(&self) -> f64 {
        self.weapon.attack_interval()
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    attack: isize,
    strength: isize,
    attack_potion: AttackPotion,
    attack_prayer: AttackPrayer,
    attack_equipment_bonus: isize,
    strength_potion: StrengthPotion,
    strength_prayer: StrengthPrayer,
    strength_equipment_bonus: isize,
    pub attack_style: AttackStyle,
    pub gear: Gear,
}

impl Player {
    pub fn new(
        name: &str,
        attack: isize,
        strength: isize,
        attack_potion: AttackPotion,
        attack_equipment_bonus: isize,
        attack_prayer: AttackPrayer,
        strength_potion: StrengthPotion,
        strength_equipment_bonus: isize,
        strength_prayer: StrengthPrayer,
        attack_style: AttackStyle,
        gear: Gear,
    ) -> Self {
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

    fn strength_style_bonus(&self, attack_style: &AttackStyle) -> isize {
        match attack_style {
            AttackStyle::ACCURATE => 0,
            AttackStyle::AGGRESSIVE => 3,
            AttackStyle::CONTROLLED => 1,
            AttackStyle::DEFENSIVE => 0,
            _ => 0,
        }
    }

    pub fn weapon_styles(&self) -> Vec<(AttackStyle, DefenceStyle)> {
        self.gear
            .weapon
            .weapon
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

    pub fn max_hit(&self, monster: &Monster, on_task: bool, attack_style: &AttackStyle) -> isize {
        let hit = 0.5
            + self.effective_strength_level(attack_style) as f64
                * (self.strength_equipment_bonus + 64) as f64
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
    ) -> isize {
        let roll = self.effective_attack_level(attack_style) * (self.attack_equipment_bonus + 64);
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
        style: &(AttackStyle, DefenceStyle),
    ) -> f64 {
        let attack = self.max_attack_roll(monster, on_task, &style.0) as f64;
        let defence = monster.max_defence_roll(&style.1) as f64;

        if attack > defence {
            1.0 - (defence + 2.0) / (2.0 * (attack + 1.0))
        } else {
            attack / (2.0 * defence + 1.0)
        }
    }

    pub fn dps(
        &self,
        monster: &Monster,
        on_task: bool,
        style: &(AttackStyle, DefenceStyle),
    ) -> f64 {
        self.hit_chance(monster, on_task, style)
            * (self.max_hit(monster, on_task, &style.0) as f64 / 2.0)
            / self.gear.attack_interval()
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DefenceStyle {
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

    fn defence_equipment_bonus(&self, defence_style: &DefenceStyle) -> isize {
        match defence_style {
            DefenceStyle::STAB => self.defence_stab,
            DefenceStyle::SLASH => self.defence_slash,
            DefenceStyle::CRUSH => self.defence_crush,
            DefenceStyle::RANGED => self.defence_ranged,
            DefenceStyle::SPELLCASTING | DefenceStyle::DEFENSIVECASTING | DefenceStyle::MAGIC => {
                self.defence_magic
            }
        }
    }

    fn max_defence_roll(&self, defence_style: &DefenceStyle) -> isize {
        self.effective_defence_level() * (self.defence_equipment_bonus(defence_style) + 64)
    }

    fn is_undead(&self) -> bool {
        self.attributes.contains(&String::from("undead"))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeaponStance {
    combat_style: String,
    pub attack_type: Option<DefenceStyle>,
    pub attack_style: Option<AttackStyle>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct _Weapon {
    pub attack_speed: isize,
    pub stances: Vec<WeaponStance>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub weapon: _Weapon,
}

impl Weapon {
    fn attack_interval(&self) -> f64 {
        self.weapon.attack_speed as f64 * 0.6
    }

    fn attack_type(&self, attack_style: usize) -> &DefenceStyle {
        &self.weapon.stances[attack_style]
            .attack_type
            .as_ref()
            .unwrap()
    }
}
