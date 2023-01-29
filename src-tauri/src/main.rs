#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



mod calculator;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calculate(input: &str) -> String {
    let tokens = calculator::Calculator::parse(input);
    println!("{:?}", tokens);
    let expr = calculator::Calculator::expression(tokens.unwrap());
    println!("{:?}", expr);
    let value = calculator::Calculator::evaluate(expr);
    println!("{}", value.unwrap());

    format!("Result: {}", value.unwrap())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
