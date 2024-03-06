use std::{thread, time};

fn main() {
    let art = [
        "            _                  _    _      _ ",
        "  _ __  _ _| |__  ___ _ _ __ _| |_ (_)_ _ (_)",
        " | '  \\| '_| '_ \\/ _ \\ '_/ _` | ' \\| | ' \\| |",
        " |_|_|_|_| |_.__/\\___/_| \\__, |_||_|_|_||_|_|",
        "                         |___/               ",
    ];

    for line in art {
        println!("{}", line);
        thread::sleep(time::Duration::from_millis(25))
    }
}
