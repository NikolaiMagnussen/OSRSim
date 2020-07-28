use crate::{Item, Monster, Weapon};
use serde::{Deserialize};
use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;
use std::collections::HashMap;

pub trait Store {
    fn connect(path: &str) -> Self;
    fn get_weapon(&self, name: &str) -> Option<&Weapon>;
    fn get_item(&self, name: &str) -> Option<&Item>;
    fn get_monster(&self, name: &str) -> Option<&Monster>;
}

#[allow(dead_code)]
pub struct FileStore {
    path: String,
    weapons: HashMap<String, Weapon>,
    items: HashMap<String, Item>,
    monsters: HashMap<String, Monster>,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    _items: Vec<T>,
}

pub struct ApiStore {
    path: String,
}

impl ApiStore {
    pub fn connect(path: &str) -> Self {
        ApiStore {
            path: String::from(path),
        }
    }


    pub async fn get_weapon(&self, name: &str) -> Result<Weapon, Box<dyn std::error::Error>> {
        let weapons = reqwest::get(&format!(
                r#"{}/weapons?where={{ "name": "{}", "duplicate": false }}"#,
                self.path, name))
            .await?
            .json::<Response<Weapon>>()
            .await?;

        if weapons._items.len() > 0 {
            Ok(weapons._items[0].clone())
        } else {
            Err(Box::new(Error::new(ErrorKind::InvalidData, "The weapon does not exist..")))
        }
    }

    pub async fn get_item(&self, name: &str) -> Result<Item, Box<dyn std::error::Error>> {
        let items = reqwest::get(&format!(
                r#"{}/items?where={{ "name": "{}", "duplicate": false }}"#,
                self.path, name))
            .await?
            .json::<Response<Item>>()
            .await?;

        if items._items.len() > 0 {
            Ok(items._items[0].clone())
        } else {
            Err(Box::new(Error::new(ErrorKind::InvalidData, "The item does not exist..")))
        }
    }

    pub async fn get_monster(&self, name: &str) -> Result<Monster, Box<dyn std::error::Error>> {
        let monster = reqwest::get(&format!(
                r#"{}/monsters?where={{ "name": "{}", "duplicate": false }}"#,
                self.path, name))
            .await?
            .json::<Response<Monster>>()
            .await?;

        if monster._items.len() > 0 {
            Ok(monster._items[0].clone())
        } else {
            Err(Box::new(Error::new(ErrorKind::InvalidData, "The monster does not exist..")))
        }
    }
}

impl Store for FileStore {
    fn connect(path: &str) -> Self {
        // Read and parse complete item file
        let file = File::open(format!("{}/items-complete.json", path)).expect("Unable to open items file");
        let reader = BufReader::new(file);
        let all_items: HashMap<String, serde_json::Value> = serde_json::from_reader(reader).expect("Unable to parse file");

        // Split item file into weapon and (equipable) items
        let mut weapons = HashMap::new();
        let mut items = HashMap::new();
        for val in all_items.values() {
            if !val["weapon"].is_null() {
                weapons.insert(String::from(val["name"]
                                                .as_str()
                                                .expect("Unable to get string from JSON object"))
                                            , serde_json::from_value(val.clone())
                                                .expect("Unable to parse weapon value"));
            } else if !val["equipment"].is_null() {
                items.insert(String::from(val["name"]
                                              .as_str()
                                              .expect("Unable to get string from JSON object"))
                                            , serde_json::from_value(val.clone())
                                                .expect("Unable to parse item value"));
            }
        }
        // Parse monster file and transform the temporary map
        let monster_file = File::open(format!("{}/monsters-complete.json", path))
            .expect("Unable to open monsters file");
        let monster_reader = BufReader::new(monster_file);
        let monsters_tmp: HashMap<String, Monster> = serde_json::from_reader(monster_reader)
            .expect("Unable to parse file");
        let monsters_transformed: HashMap<String, Monster> = monsters_tmp
            .iter()
            .map(|(_x,y)| (y.name.clone(), y.clone()))
            .collect();

        FileStore {
            path: String::from(path),
            weapons: weapons,
            items: items,
            monsters: monsters_transformed,
        }
    }


    fn get_weapon(&self, name: &str) -> Option<&Weapon> {
        self.weapons.get(name)
    }

    fn get_item(&self, name: &str) -> Option<&Item> {
        self.items.get(name)
    }

    fn get_monster(&self, name: &str) -> Option<&Monster> {
        self.monsters.get(name)
    }
}
