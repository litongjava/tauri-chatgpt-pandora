#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

use std::net::TcpStream;
use std::thread;
use std::process::Command;

fn main() {
  println!("starting");
  // 检查端口8018是否在使用
  let is_port_in_use = TcpStream::connect("127.0.0.1:8018").is_ok();
  println!("8018 port used:{}", is_port_in_use);

  if !is_port_in_use {
    thread::spawn(|| {
      let mut command = Command::new("./pandora-cloud");

      #[cfg(windows)]
      command.creation_flags(CREATE_NO_WINDOW);

      command.spawn().expect("failed to execute pandora-cloud");
    });
  } else {
    println!("Port 8018 is already in use. Not starting pandora-cloud.");
  }

  println!("tauri starting");
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
