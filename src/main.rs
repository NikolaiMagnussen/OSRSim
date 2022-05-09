use serde::Deserialize;
use tracing::{info, span, Level, error, warn};
use tracing_subscriber;

use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
mod store;

#[allow(dead_code)]
mod player;
use player::{AttackPotion, AttackPrayer, EquipmentSlot, Gear, StrengthPotion, StrengthPrayer};

#[allow(dead_code)]
mod simulation;

#[derive(Deserialize, Debug, Clone)]
struct ParsedFile {
    player_name: String,
    attack_level: isize,
    strength_level: isize,
    monster_name: String,
    equipment: Vec<String>,
}

fn load_player(
    filename: &str,
    api: &impl store::Store,
) -> Option<(player::Player, player::Monster)> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);
    let parsed_file: ParsedFile = serde_json::from_reader(reader).ok()?;

    let mut player = player::Player::new(
        &parsed_file.player_name,
        parsed_file.attack_level,
        parsed_file.strength_level,
        AttackPotion::NONE,
        AttackPrayer::NONE,
        StrengthPotion::NONE,
        StrengthPrayer::NONE,
        Gear::empty(),
    );

    // Parse all equipment
    for eq in &parsed_file.equipment{
        let weapon= api.get_weapon(&eq);
        let item = api.get_item(&eq);

	// Add armour and weapons, but warn if equipment was not matched.
        match (&weapon, &item) {
            (None, None) => warn!("Warning: {} was not matched :(", eq),
            (Some(_), None) => player.equipment.add_weapon(weapon.as_ref()),
            (None, Some(_)) => player.equipment.add_equipment(item.as_ref()),
            (Some(_), Some(_)) => error!("This should not happen!"),
        }
    }
    info!("Test end of stuff");

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
    tracing_subscriber::fmt::init();
    info!("Loading store..");
    let api: store::FileStore = store::Store::connect("osrsbox-db");

    info!("Store loaded..");
    if let Some((player, monster)) = load_player("./loadout.json", &api) {
        info!("Attack styles: {:#?}", simulation::run_attack_styles(&player, &monster));
        let better = simulation::run(player, &monster);
        info!("Better player: {:#?}", better);
    } else {
        error!("Unable to parse loadout :(");
    }

    Ok(())
}
