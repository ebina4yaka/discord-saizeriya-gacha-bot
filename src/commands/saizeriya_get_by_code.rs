use crate::data;
use crate::data::menu::Menu;
use data::menu;
use itertools::Itertools;
use serenity::all::{CommandOptionType, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption]) -> String {
    let code = get_code(options);

    match menu::take_menu(code) {
        None => "メニューが見つかりませんでした。".to_string(),
        Some(menu) => generate_result_text(menu),
    }
}

fn generate_result_text(m: Menu) -> String {
    let mut results: Vec<String> = vec!["```".to_string()];
    let menu = m.clone();
    let id = &menu.id;
    let name = &menu.name;
    let value = &menu.value.to_string();
    let res = [id, ": ", name, " ", value, "円\n"];
    let text = res.iter().join("");
    results.push(text);
    results.push("```".to_string());

    results.iter().join("")
}

fn get_code(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        name: "number",
        value: ResolvedValue::String(id),
        ..
    }) = options.first()
    {
        return (*id).parse().unwrap();
    }

    "".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("saizeriya_get_by_code")
        .description("番号からサイゼリヤのメニューを取得します")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "code", "注文番号").required(true),
        )
}
