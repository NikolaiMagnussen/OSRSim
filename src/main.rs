use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
mod store;
use store::Store;

#[allow(dead_code)]
mod player;
use player::{
    AttackPotion, AttackPrayer, AttackStyle, Gear, HeadSlot, NeckSlot, SetBonus, StrengthPotion,
    StrengthPrayer,
};

#[allow(dead_code)]
mod simulation;

#[derive(Deserialize, Debug, Clone)]
struct ParsedOneHand {
    weapon: String,
    shield: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum ParsedWeapon {
    ONEHAND(ParsedOneHand),
    TWOHAND(String),
}

#[derive(Deserialize, Debug, Clone)]
struct ParsedEquipment {
    ring: String,
    feet: String,
    hands: String,
    neck: String,
    ammo: String,
    cape: String,
    body: String,
    legs: String,
    head: String,
    weapon: ParsedWeapon,
}

#[derive(Deserialize, Debug, Clone)]
struct ParsedFile {
    player_name: String,
    attack_level: isize,
    attack_bonus: isize,
    strength_level: isize,
    strength_bonus: isize,
    weapon_name: String,
    slayer_helm: bool,
    monster_name: String,
    equipment: ParsedEquipment,
}

fn load_player(
    filename: &str,
    api: &impl store::Store,
) -> Option<(player::Player, player::Monster)> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);
    let parsed_file: ParsedFile = serde_json::from_reader(reader).ok()?;

    let weapon = api.get_weapon(&parsed_file.weapon_name)?;
    let mut player = player::Player::new(
        &parsed_file.player_name,
        parsed_file.attack_level,
        parsed_file.strength_level,
        AttackPotion::NONE,
        parsed_file.attack_bonus,
        AttackPrayer::NONE,
        StrengthPotion::NONE,
        parsed_file.strength_bonus,
        StrengthPrayer::NONE,
        AttackStyle::ACCURATE,
        Gear::new(
            SetBonus::NONE,
            HeadSlot::SLAYER,
            NeckSlot::NONE,
            weapon.clone(),
        ),
    );
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.ring));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.feet));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.hands));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.neck));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.ammo));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.cape));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.body));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.legs));
    player.gear.add_equipment(api.get_item(&parsed_file.equipment.head));
    match parsed_file.equipment.weapon {
        ParsedWeapon::TWOHAND(twohand) => player.gear.add_equipment(api.get_item(&twohand)),
        ParsedWeapon::ONEHAND(onehand) => {
            player.gear.add_equipment(api.get_item(&onehand.shield));
            player.gear.add_weapon(api.get_weapon(&onehand.weapon));
        },
    };

    let monster = api.get_monster(&parsed_file.monster_name)?;
    Some((player, monster.clone()))
}

#[allow(dead_code)]
async fn do_hardcoded() -> Result<(), Box<dyn std::error::Error>> {
    let monster_name = "Aberrant spectre";
    let weapon_name = "Abyssal whip";

    let api = store::ApiStore::connect("https://api.osrsbox.com");
    let weapon = api.get_weapon(weapon_name).await?;
    let monster = api.get_monster(monster_name).await?;

    let player = player::Player::new(
        "Supergeni",
        74,
        74,
        AttackPotion::ATTACK,
        132,
        AttackPrayer::NONE,
        StrengthPotion::STRENGTH,
        110,
        StrengthPrayer::NONE,
        AttackStyle::CONTROLLED,
        Gear::new(SetBonus::NONE, HeadSlot::SLAYER, NeckSlot::NONE, weapon),
    );

    let better = simulation::run(player.clone(), &monster);
    println!("Better player: {:#?}", better);

    // Compared to osrs-genie.com - we have the correct DPS, because the author
    // of the tool has not divided the max hit by two to get the average hit
    // resulting in the DPS being double.
    println!(
        "{} has {} DPS against {}",
        player.name,
        player.dps(&monster, true, player.weapon_styles().first().unwrap()),
        monster.name
    );
    println!("Getting the shark: {:#?}", api.get_item("Shark").await?);

    Ok(())
}

/* What modules to have:
 * - main (orchestrate everything - for now)
 * - store (for querying items, via API, parsed file or other way)
 * - player (for handling everything with an instance of a player)
 * - simulation (for generating "player instances" for evaulation and comparing results)
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading store..");
    let api: store::FileStore = store::Store::connect("osrsbox-db");

    println!("Store loaded..");
    if let Some((player, monster)) = load_player("./loadout.json", &api) {
        println!("Loaded player: {:#?}", player);
        let better = simulation::run(player, &monster);
        println!("Better player: {:#?}", better);
    } else {
        println!("Unable to parse loadout :(");
    }

    Ok(())
}
