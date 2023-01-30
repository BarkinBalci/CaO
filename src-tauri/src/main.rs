#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



mod calculator;
use crate::calculator::Calculator;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calculate(input: &str) -> String {
    let tokens = Calculator::parse(input);
    let expr = Calculator::expression(tokens.unwrap());
    let value = Calculator::evaluate(expr);
    match value{
        None => format!(""),
        Some(..) => format!("{}", value.unwrap()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
