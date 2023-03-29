#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use tauri::command;
use winsafe::GetUserName;
use lnk::ShellLink;
use std::fs;

fn test() {
  unsafe {

  }
}

#[command]
fn tester() -> Vec<String> {
    let user: String = match GetUserName() {
      Ok(username) => {
        username
      }
      // handle better when working
      Err(e) => {
        String::from("test")
      }
    };

    let path = format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar", user);
    let entries = fs::read_dir(path).unwrap();
    let mut lnks: Vec<String> = vec![];
    let mut shrt_lnks: Vec<String> = vec![];
    for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path();
      let pathStr = match path.to_str() {
        Some(str) => str.to_string(),
        None => String::from("Something went wrong with this path u dumbfuck")
      };
      if pathStr.contains("lnk") {
        lnks.push(pathStr);
      }
    }
    println!("{:?}", lnks);
    for link in lnks {
      let shortcut = lnk::ShellLink::open(link).unwrap();
      let name = match shortcut.icon_location() {
        Some(link) => {
          link
        }
        None => {
          "bad"
        }
      };
      shrt_lnks.push(name.to_string())
    }
    return shrt_lnks;
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
      tester,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
