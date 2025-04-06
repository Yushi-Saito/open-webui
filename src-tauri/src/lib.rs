use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// グローバル変数としてPythonプロセスを保持
struct AppState {
    python_process: Option<Child>,
}

pub fn run() {
    // アプリケーションの状態を初期化
    let app_state = Arc::new(Mutex::new(AppState {
        python_process: None,
    }));

    // クローンを作成
    let app_state_for_window = app_state.clone();

    tauri::Builder::default()
        .setup(move |_app| {
            // アプリケーションの状態をクローン
            let app_state_clone = app_state.clone();
            
            // Pythonバックエンドを起動
            let python_command = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/c", "cd", "backend", "&&", "python", "-m", "open_webui.main"])
                    .spawn()
            } else {
                Command::new("sh")
                    .args(&["-c", "cd backend && python -m open_webui.main"])
                    .spawn()
            };

            match python_command {
                Ok(process) => {
                    println!("Pythonバックエンドを起動しました");
                    let mut state = app_state_clone.lock().unwrap();
                    state.python_process = Some(process);
                }
                Err(e) => {
                    eprintln!("Pythonバックエンドの起動に失敗しました: {}", e);
                }
            }

            // バックエンドサーバーが起動するのを少し待つ
            thread::sleep(Duration::from_secs(2));

            Ok(())
        })
        .on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                // ウィンドウが閉じられたときにPythonプロセスを終了
                let mut state = app_state_for_window.lock().unwrap();
                if let Some(mut process) = state.python_process.take() {
                    println!("Pythonバックエンドを終了しています...");
                    let _ = process.kill();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
