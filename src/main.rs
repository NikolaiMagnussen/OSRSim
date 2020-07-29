use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
mod store;
use store::Store;

#[allow(dead_code)]
mod player;
use player::{
    AttackPotion, AttackPrayer, AttackStyle, Gear, StrengthPotion, StrengthPrayer,
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
    strength_level: isize,
    monster_name: String,
    equipment: ParsedEquipment,
    spare_equipment: Vec<String>,
}

fn load_player(
    filename: &str,
    api: &impl store::Store,
) -> Option<(player::Player, player::Monster)> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);
    let parsed_file: ParsedFile = serde_json::from_reader(reader).ok()?;
    println!("Parsed file: {:#?}", parsed_file);

    let mut player = player::Player::new(
        &parsed_file.player_name,
        parsed_file.attack_level,
        parsed_file.strength_level,
        AttackPotion::NONE,
        AttackPrayer::NONE,
        StrengthPotion::NONE,
        StrengthPrayer::NONE,
        AttackStyle::ACCURATE,
        Gear::empty(),
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

    for equipment in parsed_file.spare_equipment {
        player.spare_equipment.add_weapon(api.get_weapon(&equipment));
        player.spare_equipment.add_equipment(api.get_item(&equipment));
    }

    println!("Parsed player: {:#?}", player);

    let monster = api.get_monster(&parsed_file.monster_name)?;
    Some((player, monster.clone()))
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
        let better = simulation::run(player, &monster);
        println!("Better player: {:#?}", better);
    } else {
        println!("Unable to parse loadout :(");
    }

    Ok(())
}
