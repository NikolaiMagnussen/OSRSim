use crate::{Item, Monster, Weapon};
use serde::{Deserialize};
use std::io::{Error, ErrorKind};
//use std::collections::HashMap;

pub trait Store {
    fn connect(path: &str) -> Self;
    fn get_weapon(&self, name: &str) -> Option<Weapon>;
    fn get_item(&self, name: &str) -> Option<Item>;
    fn get_monster(&self, name: &str) -> Option<Monster>;
}

/*
pub struct FileStore {
    path: String,
    weapons: HashMap<String, Weapon>,
    items: HashMap<String, Item>,
    monsters: HashMap<String, Monster>,
}
*/

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

/*
impl Store for FileStore {
    fn connect(path: &str) -> Self {}

    fn get_weapon(&self, name: &str) -> Weapon {}

    fn get_item(&self, name: &str) -> Item {}

    fn get_monster(&self, name: &str) -> Monster {}
}
*/
