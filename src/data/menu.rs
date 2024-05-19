use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader};

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

fn load_menu_from_json_file() -> Result<Vec<Menu>, Box<dyn Error>> {
    let file = File::open("./src/data/menu.json")?;
    let reader = BufReader::new(file);
    let menu = serde_json::from_reader(reader)?;
    Ok(menu)
}

pub fn take_menu(id: String) -> Option<Menu> {
    let menus = load_menu_from_json_file().unwrap();

    menus.into_iter().find(|m| m.id == id)
}

pub fn pick_random_menus(params: GetRandomMenuParams) -> Vec<Menu> {
    let menus = load_menu_from_json_file().unwrap();
    let min_value = get_min(&menus);

    let mut picked_menus: Vec<Menu> = vec![];

    loop {
        let menu = pick_rand_menu(menus.clone());

        if menu.category == "alcohol" && params.exclude_alcohol {
            continue;
        }

        picked_menus.push(menu);

        if picked_menus.iter().map(|i| i.value).sum::<i64>() > params.price_limit {
            picked_menus.pop();
        }

        if params.price_limit - picked_menus.iter().map(|i| i.value).sum::<i64>() < min_value {
            return picked_menus;
        }
    }
}

fn pick_rand_menu(mut menus: Vec<Menu>) -> Menu {
    let mut rng = rand::thread_rng();
    menus.shuffle(&mut rng);
    let m = menus.first().unwrap();

    m.clone()
}

fn get_min(slice: &[Menu]) -> i64 {
    let mut min = &slice[0].value;

    for item in slice {
        if item.value < *min {
            min = &item.value;
        }
    }

    *min
}

#[cfg(test)]
mod tests {
    use crate::data::menu::{get_min, load_menu_from_json_file, take_menu, Menu};

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

    #[test]
    fn test_get_min() {
        let menus = load_menu_from_json_file().unwrap();
        let want = 100;

        let actual = get_min(&menus);

        assert_eq!(actual, want)
    }
}
