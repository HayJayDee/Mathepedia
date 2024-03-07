// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;
mod models;

use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

const DATA: &str = "When <math><mi>a</mi><mo>&#x2260;</mo><mn>0</mn></math>, \
there are two solutions to <math>\
<mi>a</mi><msup><mi>x</mi><mn>2</mn></msup>\
<mo>+</mo> <mi>b</mi><mi>x</mi>\
<mo>+</mo> <mi>c</mi> <mo>=</mo> <mn>0</mn>\
</math> and they are\
<math mode=\"display\">\
<mi>x</mi> <mo>=</mo>\
<mrow class=\"reference\" href=\"index.html?id=1\">\
    <mfrac>\
    <mrow>\
        <mo>&#x2212;</mo>\
        <mi>b</mi>\
        <mo>&#x00B1;</mo>\
        <msqrt>\
        <msup><mi>b</mi><mn>2</mn></msup>\
        <mo>&#x2212;</mo>\
        <mn>4</mn><mi>a</mi><mi>c</mi>\
        </msqrt>\
    </mrow>\
    <mrow> <mn>2</mn><mi>a</mi> </mrow>\
    </mfrac>\
</mrow>\
<mtext>.</mtext>\
</math>";

const DATA1: &str = "Test";

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Could not found database_url");
    PgConnection::establish(&database_url).unwrap_or_else( |err| panic!("Error connecting to {}", err))
}

#[tauri::command]
fn get_reference_data(id: i32) -> String {
    println!("{}", id);
    match id {
        0 => DATA.to_string(),
        1 => DATA1.to_string(),
        _ => "ERROR".to_string()
    }
}

fn main() {

    establish_connection();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_reference_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
