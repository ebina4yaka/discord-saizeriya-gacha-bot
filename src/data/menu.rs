use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, path::Path};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Menu {
    pub id: String,
    pub name: String,
    pub value: i64,
    category: String,
}

pub struct GetRandomMenuParams {
    pub price_limit: i64,
    pub exclude_alcohol: bool,
}

fn load_menu_from_json_file<P: AsRef<Path>>(path: P) -> Result<Vec<Menu>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let menu = serde_json::from_reader(reader)?;
    Ok(menu)
}

pub fn take_menu(id: String) -> Option<Menu> {
    let menus = load_menu_from_json_file("./src/data/menu.json").unwrap();

    menus.into_iter().find(|m| m.id == id)
}

pub fn pick_random_menus(params: GetRandomMenuParams) -> Vec<Menu> {
    let mut menus = load_menu_from_json_file("./src/data/menu.json").unwrap();

    let mut rng = rand::thread_rng();
    menus.shuffle(&mut rng);

    let mut picked_menus: Vec<Menu> = vec![];

    for m in menus {
        if m.category == "alcohol" && params.exclude_alcohol {
            continue;
        }
        picked_menus.push(m);
        if picked_menus.iter().map(|i| i.value).sum::<i64>() > params.price_limit {
            picked_menus.pop();
        }
    }

    picked_menus
}

#[cfg(test)]
mod tests {
    use crate::data::menu::{take_menu, Menu};

    #[test]
    fn test_take_menu() {
        let arg = "304";
        let option_menu = take_menu(arg.to_string());

        let want = serde_json::to_string(&Menu {
            id: "304".to_string(),
            name: "エクストラ・バージン・オリーブオイル".to_string(),
            value: 1200,
            category: "takeout".to_string(),
        })
        .unwrap();

        match option_menu {
            None => panic!("missing menu"),
            Some(m) => {
                let actual = serde_json::to_string(&m).unwrap();
                assert_eq!(actual, want);
            }
        }
    }
}
