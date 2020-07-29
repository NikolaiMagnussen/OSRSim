#[allow(dead_code)]
use crate::player::{AttackStyle, AttackType, Gear, Monster, Player, SpareGear};

pub struct Simulation {
    gear: Gear,
    spare_equipment: SpareGear,
}

impl Simulation {
    pub fn new(gear: &Gear, spare_equipment: &SpareGear) -> Self {
        Simulation {
            gear: gear.clone(),
            spare_equipment: spare_equipment.clone(),
        }
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

pub fn run(base: Player, monster: &Monster) -> (Player, (f64, (AttackStyle, AttackType))) {
    let mut player = base.clone();

    let mut style: (f64, (AttackStyle, AttackType)) = run_attack_styles(&player, monster);
    let mut best_style = style;
    for equipment in &base.spare_equipment.spare_equipment {
        let mut player_tmp = player.clone();
        for weapon in &base.spare_equipment.spare_weapons {
            player_tmp.gear.add_weapon(Some(weapon));
            player_tmp.gear.add_equipment(Some(equipment));

            /* Run sim! */
            style = run_attack_styles(&player_tmp, monster);
            println!("Style and DPS: {:#?}", style);

            /* Update best if the new sim is better */
            if style.0 > best_style.0 {
                best_style = style;
                player = player_tmp.clone();
            }
        }
    }
    println!(
        "The best style and player yields: {:#?}\n{:#?}",
        player, best_style
    );

    (player, best_style)
}
