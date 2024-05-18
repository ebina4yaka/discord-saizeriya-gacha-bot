use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{error::Error, fs::File, io::BufReader, path::Path};

#[derive(Deserialize, Clone)]
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

pub fn pick_random_menus(params: GetRandomMenuParams) -> Vec<Menu> {
    let mut menus = load_menu_from_json_file("menu.json").unwrap();

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
