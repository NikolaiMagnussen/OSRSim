#[allow(dead_code)]
use crate::player::{
    AttackPotion, AttackPrayer, AttackStyle, DefenceStyle, Gear, Monster,
    Player, StrengthPotion, StrengthPrayer,
};

pub fn run_attack_styles(base: &Player, monster: &Monster) -> (f64, (AttackStyle, DefenceStyle)) {
    let mut a: Vec<(f64, (AttackStyle, DefenceStyle))> = base
        .weapon_styles()
        .iter()
        .map(|x| (base.dps(monster, true, x), x.clone()))
        .collect();
    a.sort_unstable_by(|x, y| y.0.partial_cmp(&x.0).unwrap());

    a.first().unwrap().clone()
}

pub fn run(base: Player, monster: &Monster) -> Player {
    let mut player = base.clone();
    let style: (f64, (AttackStyle, DefenceStyle)) = run_attack_styles(&base, monster);
    println!("Style and DPS: {:#?}", style);

    player.attack_style = (style.1).0;

    player
}
