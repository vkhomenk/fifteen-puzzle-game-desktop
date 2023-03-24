// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fifteen_solver;
use fifteen_solver::{solver, validate};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[tauri::command]
async fn solve(tiles: Vec<u8>) -> String {
    match solver(tiles) {
        Ok(solution) => {
            println!("{solution}");
            solution
        },
        Err(err) => {
            println!("{err}");
            err
        },
    }
}

#[tauri::command]
fn random_tiles() -> Vec<u8> {
    let mut tiles: Vec<u8> = (0..16).collect();
    loop {
        tiles.shuffle(&mut thread_rng());
        if validate(&tiles).is_ok() {
            return tiles;
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![solve, random_tiles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
