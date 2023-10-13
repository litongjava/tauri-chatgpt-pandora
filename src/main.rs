#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

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
      let output = Command::new("./pandora-cloud")
        .output()
        .expect("failed to execute pandora-cloud");

      // 输出命令的结果
      println!("Status: {}", output.status);
      println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    });
  } else {
    println!("Port 8018 is already in use. Not starting pandora-cloud.");
  }

  println!("tauri starting");
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
