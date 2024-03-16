use command_group::CommandGroup;
use std::process::Command;
use std::sync::mpsc::Receiver;
use std::thread;
use tauri::api::process::Command as TCommand;

// 启动后端
pub fn start_backend(receiver: Receiver<i32>) {
    // `new_sidecar()` expects just the filename, NOT the whole path
    let t = TCommand::new_sidecar("main").expect("[Error] 无法创建“main.exe”二进制命令");
    let mut group = Command::from(t)
        .group_spawn()
        .expect("[Error] 生成 API 服务器进程");
    thread::spawn(move || loop {
        let s = receiver.recv();
        if s.unwrap() == -1 {
            group.kill().expect("[Error] 终止 API 服务器进程");
        }
    });
}
