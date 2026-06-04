use std::net::TcpListener;

fn main() {

    let listener =
        TcpListener::bind(
            "127.0.0.1:7000"
        )
        .unwrap();

    println!(
        "monitor listening on 127.0.0.1:7000"
    );

    let (stream, _) =
        listener.accept()
            .unwrap();

    println!("experiment connected");

    trace::ui::ui_loop::run_ui_loop(

        stream,

        0,
        0,
        0,
    );

    println!("monitor finished");
}