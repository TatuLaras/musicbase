use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    process::Command,
};

pub fn start_mpv_process() -> Result<(), ()> {
    Command::new("mpv")
        .arg("--no-audio-display")
        .arg("--idle")
        // .arg("--terminal=no")
        .arg("--input-ipc-server=/tmp/mpvsocket")
        .spawn()
        .unwrap();
    Ok(())
}

// Plays file at path using the default audio device.
// If parameter queue is true, the file will be added to a queue to be played gaplessly.
pub fn play_file(path: &str, queue: bool) {
    let command = format!(
        "{{ \"command\": [\"loadfile\", \"{}\", \"append-play\"] }}\n",
        path
    );
    let result = send_command(command);
    println!("result: {}", result);
}

fn send_command(command: String) -> String {
    let mut unix_stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
    unix_stream.write(command.as_bytes()).unwrap();
    unix_stream.shutdown(Shutdown::Write).unwrap();
    let mut response = String::new();
    unix_stream.read_to_string(&mut response).unwrap();
    response
}
