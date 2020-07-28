#[allow(dead_code)]
mod store;

#[allow(dead_code)]
mod player;
use player::{
    AttackPotion, AttackPrayer, AttackStyle, Gear, HeadSlot, NeckSlot, SetBonus, StrengthPotion,
    StrengthPrayer,
};

#[allow(dead_code)]
mod simulation;

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

    /*
     *  Example of using the file store API to fetch information
     *  about weapons, (equipable) items and monsters.
     */
    use store::Store;
    let f: store::FileStore = store::Store::connect("osrsbox-db");
    let weapon = f.get_weapon(weapon_name);
    let monster = f.get_monster(monster_name);
    let item = f.get_item("Dragon chainbody");

    println!("file store weapon: {:#?}", weapon);
    println!("file store monster: {:#?}", monster);
    println!("file store item: {:#?}", item);
    //*/

    Ok(())
}
