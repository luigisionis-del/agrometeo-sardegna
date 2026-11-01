// Evita la console aggiuntiva su Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    agrometeo_lib::run()
}
