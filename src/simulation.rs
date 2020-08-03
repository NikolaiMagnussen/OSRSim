use std::collections::HashSet;

use crate::player::{
    AttackStyle, AttackType, Equipment, EquipmentSlot, Gear, Monster, Player, SpareGear, Weapon,
};

// Interesting optimalization:
// - ignore everything of gear that does not provide any attack bonuses
// - except for void, salve and slayer that provides special bonuses

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GearSet {
    ammo: Option<Equipment>,
    body: Option<Equipment>,
    cape: Option<Equipment>,
    feet: Option<Equipment>,
    head: Option<Equipment>,
    legs: Option<Equipment>,
    neck: Option<Equipment>,
    ring: Option<Equipment>,
    hands: Option<Equipment>,
    shield: Option<Equipment>,
    weapon: Option<Weapon>,
}

impl GearSet {
    fn new(ammo: Option<Equipment>,
    body: Option<Equipment>,
    cape: Option<Equipment>,
    feet: Option<Equipment>,
    head: Option<Equipment>,
    legs: Option<Equipment>,
    neck: Option<Equipment>,
    ring: Option<Equipment>,
    hands: Option<Equipment>,
    shield: Option<Equipment>,
    weapon: Option<Weapon>) -> Self {
        GearSet {
      ammo: ammo,
      body: body,
      cape: cape,
      feet: feet,
      head: head,
      legs: legs,
      neck: neck,
      ring: ring,
     hands: hands,
    shield: shield,
    weapon: weapon,
        }
    }

    fn equip_player(&self, base: &Player) -> Player {
        let mut p = base.clone();
        p.gear.add_equipment(self.ammo.as_ref());
        p.gear.add_equipment(self.body.as_ref());
        p.gear.add_equipment(self.cape.as_ref());
        p.gear.add_equipment(self.feet.as_ref());
        p.gear.add_equipment(self.head.as_ref());
        p.gear.add_equipment(self.legs.as_ref());
        p.gear.add_equipment(self.neck.as_ref());
        p.gear.add_equipment(self.ring.as_ref());
        p.gear.add_equipment(self.hands.as_ref());
        p.gear.add_equipment(self.shield.as_ref());
        p.gear.add_weapon(self.weapon.as_ref());
        p
    }
}

#[derive(Debug)]
pub struct Simulation {
    gear: Gear,
    original_gear: Gear,
    spare_equipment: SpareGear,
    ammo: HashSet<Equipment>,
    body: HashSet<Equipment>,
    cape: HashSet<Equipment>,
    feet: HashSet<Equipment>,
    head: HashSet<Equipment>,
    legs: HashSet<Equipment>,
    neck: HashSet<Equipment>,
    ring: HashSet<Equipment>,
    hands: HashSet<Equipment>,
    shield: HashSet<Equipment>,
    weapon: HashSet<Weapon>,
    twohand: HashSet<Weapon>,
}

impl Simulation {
    pub fn new(gear: &Gear, spare_equipment: &SpareGear) -> Self {
        Simulation {
            gear: gear.clone(),
            original_gear: gear.clone(),
            spare_equipment: spare_equipment.clone(),
            ammo: HashSet::new(),
            body: HashSet::new(),
            cape: HashSet::new(),
            feet: HashSet::new(),
            head: HashSet::new(),
            legs: HashSet::new(),
            neck: HashSet::new(),
            ring: HashSet::new(),
            hands: HashSet::new(),
            shield: HashSet::new(),
            weapon: HashSet::new(),
            twohand: HashSet::new(),
        }
    }

    pub fn get_gear(&self) -> &Gear {
        &self.gear
    }

    pub fn init(&mut self) {
        for (k, v) in self.gear.equipment.iter() {
            match k {
                EquipmentSlot::AMMO => self.ammo.insert(v.clone()),
                EquipmentSlot::BODY => self.body.insert(v.clone()),
                EquipmentSlot::CAPE => self.cape.insert(v.clone()),
                EquipmentSlot::FEET => self.feet.insert(v.clone()),
                EquipmentSlot::HEAD => self.head.insert(v.clone()),
                EquipmentSlot::LEGS => self.legs.insert(v.clone()),
                EquipmentSlot::NECK => self.neck.insert(v.clone()),
                EquipmentSlot::RING => self.ring.insert(v.clone()),
                EquipmentSlot::HANDS => self.hands.insert(v.clone()),
                EquipmentSlot::SHIELD => self.shield.insert(v.clone()),
                _ => true,
            };
        }

        match &self.gear.weapon.equipment.slot {
            EquipmentSlot::WEAPON => self.weapon.insert(self.gear.weapon.clone()),
            EquipmentSlot::TWOHAND => self.twohand.insert(self.gear.weapon.clone()),
            _ => true,
        };

        for v in &self.spare_equipment.spare_weapons {
            match &v.equipment.slot {
                EquipmentSlot::WEAPON => self.weapon.insert(v.clone()),
                EquipmentSlot::TWOHAND => self.twohand.insert(v.clone()),
                _ => true,
            };
        }

        for v in &self.spare_equipment.spare_equipment {
            match &v.equipment.slot {
                EquipmentSlot::AMMO => self.ammo.insert(v.clone()),
                EquipmentSlot::BODY => self.body.insert(v.clone()),
                EquipmentSlot::CAPE => self.cape.insert(v.clone()),
                EquipmentSlot::FEET => self.feet.insert(v.clone()),
                EquipmentSlot::HEAD => self.head.insert(v.clone()),
                EquipmentSlot::LEGS => self.legs.insert(v.clone()),
                EquipmentSlot::NECK => self.neck.insert(v.clone()),
                EquipmentSlot::RING => self.ring.insert(v.clone()),
                EquipmentSlot::HANDS => self.hands.insert(v.clone()),
                EquipmentSlot::SHIELD => self.shield.insert(v.clone()),
                _ => true,
            };
        }
    }

    pub fn get_gear_combinations(&self) -> HashSet<GearSet> {
        let mut set = HashSet::new();

        for ammo in &self.head {
            for body in &self.body {
                for cape in &self.cape {
                    for feet in &self.feet {
                        for head in &self.head {
                            for legs in &self.legs {
                                for neck in &self.neck {
                                    for ring in &self.ring {
                                        for hands in &self.hands {
                                            for twohand in &self.twohand {
                                                let gc = GearSet::new(
                                                Some(ammo.clone()),
                                                Some(body.clone()),
                                                Some(cape.clone()),
                                                Some(feet.clone()),
                                                Some(head.clone()),
                                                Some(legs.clone()),
                                                Some(neck.clone()),
                                                Some(ring.clone()),
                                                Some(hands.clone()),
                                                None,
                                                Some(twohand.clone()));

                                                set.insert(gc);
                                            }

                                            for weapon in &self.weapon {
                                                for shield in &self.shield {
                                                    let gc = GearSet::new(
                                                    Some(ammo.clone()),
                                                    Some(body.clone()),
                                                    Some(cape.clone()),
                                                    Some(feet.clone()),
                                                    Some(head.clone()),
                                                    Some(legs.clone()),
                                                    Some(neck.clone()),
                                                    Some(ring.clone()),
                                                    Some(hands.clone()),
                                                    Some(shield.clone()),
                                                    Some(weapon.clone()));

                                                    set.insert(gc);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        set
    }
}

pub fn run_attack_styles(base: &Player, monster: &Monster) -> (f64, (AttackStyle, AttackType)) {
    let mut a: Vec<(f64, (AttackStyle, AttackType))> = base
        .weapon_styles()
        .iter()
        .map(|x| (base.dps(monster, true, x), x.clone()))
        .collect();
    a.sort_unstable_by(|x, y| y.0.partial_cmp(&x.0).unwrap());

    a.first().unwrap().clone()
}

pub fn run(player: Player, monster: &Monster) -> ((f64, (AttackStyle, AttackType)), GearSet) {
    let mut sim = Simulation::new(&player.gear, &player.spare_equipment);
    sim.init();
    let gear = sim.get_gear_combinations();

    let mut results: Vec<((f64, (AttackStyle, AttackType)), &GearSet)> = gear
        .iter()
        .map(|x| (run_attack_styles(&x.equip_player(&player), monster), x))
        .collect();
    results.sort_unstable_by(|x, y| (y.0).0.partial_cmp(&(x.0).0).unwrap());

    let fst = results.first().expect("This should not happen: we need to have at least one gearset..");
    (fst.0, fst.1.clone())
}
