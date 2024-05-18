use crate::data;
use data::menu;
use itertools::Itertools;
use serenity::all::{CommandOptionType, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption]) -> String {
    let price_limit = get_price_limit(options);
    let exclude_alcohol = get_exclude_alcohol(options);

    let params = menu::GetRandomMenuParams {
        price_limit,
        exclude_alcohol,
    };

    let price_limit_str = price_limit.to_string();
    let mut result_texts = vec![
        "サイゼリヤ".to_string(),
        price_limit_str,
        "円ガチャ\n".to_string(),
    ];

    let menus = menu::pick_random_menus(params);
    let mut gacha_results = create_gacha_results(menus);
    result_texts.append(&mut gacha_results);


    return result_texts.iter().join("");
}

fn create_gacha_results(menus: Vec<menu::Menu>) -> Vec<String> {
    let mut results: Vec<String> = vec!["```".to_string()];
    let mut sum: i64 = 0;
    for m in menus {
        let menu = m.clone();
        let id = &menu.id;
        let name = &menu.name;
        let value = &menu.value.to_string();
        let res = [id, ": ", name, " ", value, "円\n"];
        let text = res.iter().join("");
        results.push(text);
        sum += menu.value
    }

    results.push("```".to_string());
    let sum_string = sum.to_string();
    let sum_line = ["合計", &*sum_string, "円\n"];
    results.push(sum_line.iter().join(""));

    results
}

fn get_price_limit(options: &[ResolvedOption]) -> i64 {
    let default = 1000;
    if let Some(ResolvedOption {
        name: "price_limit",
        value: ResolvedValue::Integer(price_limit),
        ..
    }) = options.first()
    {
        return *price_limit;
    }

    default
}

fn get_exclude_alcohol(options: &[ResolvedOption]) -> bool {
    let default = false;
    if let Some(ResolvedOption {
        name: "get_exclude_alcohol",
        value: ResolvedValue::Boolean(exclude_alcohol),
        ..
    }) = options.first()
    {
        return *exclude_alcohol;
    }

    default
}

pub fn register() -> CreateCommand {
    CreateCommand::new("saizeriya_gacha")
        .description("サイゼリヤガチャを回します")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "price_limit",
                "値段上限設定(デフォルトは1000円)",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "exclude_alcohol",
                "アルコール類を除くか(デフォルトはfalse)",
            )
            .required(false),
        )
}
