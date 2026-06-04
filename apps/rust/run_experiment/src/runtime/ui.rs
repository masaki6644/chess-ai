use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn spawn_ui(
    addr: String,
    total_files: usize,
    parse_workers: usize,
    label_workers: usize,
) -> JoinHandle<()> {
    thread::spawn(move || {

        let stream = loop {
            match TcpStream::connect(&addr) {
                Ok(s) => break s,
                Err(_) => {
                    std::thread::sleep(Duration::from_millis(200));
                }
            }
        };

        trace::ui::ui_loop::run_ui_loop(
            stream,
            total_files,
            parse_workers,
            label_workers,
        );
    })
}