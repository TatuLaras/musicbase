use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

static mut STREAM: Option<OutputStream> = None;
static mut STREAM_HANDLE: Option<OutputStreamHandle> = None;
static mut SINK: Option<Sink> = None;
static mut LOCKED: bool = false;

// Empties the queue and plays the sound file at path, from the same position.
// Used in situations where we need to get rid of additional queued up items
// in the sink due to the next song in queue changing
pub unsafe fn replace_with(path: &str) {
    while LOCKED {}
    LOCKED = true;
    let Some(ref sink) = SINK else {return;};
    let pos = sink.get_pos();

    play_file(path, false);
    let _ = sink.try_seek(pos).unwrap();
    LOCKED = false;
}

pub unsafe fn seek_to(millisecs: u64) {
    while LOCKED {
        println!("LOCKED: seek_to")
    }
    LOCKED = true;

    let Some(ref sink) = SINK else {return;};
    let _ = sink.try_seek(Duration::from_millis(millisecs)).unwrap();
    println!("seek {}", millisecs);

    LOCKED = false;
}

// Plays file at path using the default audio device.
// If parameter queue is true, the file will be added to a queue to be played gaplessly.
pub unsafe fn play_file(path: &str, queue: bool) {
    println!("play_file");
    while LOCKED {
        println!("LOCKED");
    }
    LOCKED = true;

    if let None = STREAM_HANDLE {
        // Get an output stream handle to the default physical sound device
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        STREAM_HANDLE = Some(stream_handle);
        STREAM = Some(stream);
    }

    let Some(ref stream_handle) = STREAM_HANDLE else {
        println!("No stream handle");

        return;};

    if let None = SINK {
        SINK = Some(Sink::try_new(&stream_handle).unwrap());
    }

    let Some(ref sink) = SINK else {
        println!("No sink");
        return;};

    let file = BufReader::new(File::open(path).unwrap());

    // Decode sound file into a source
    let source = Decoder::new(file).unwrap();

    if !queue {
        println!("CLEAR");
        sink.clear();
    }

    sink.append(source);
    println!("src: {}", path);

    // In case we paused it with sink.clear()
    sink.play();

    LOCKED = false;
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    // sink.sleep_until_end();
}

pub unsafe fn get_sink_pos() -> Duration {
    while LOCKED {}
    LOCKED = true;

    let Some(ref sink) = SINK else {return Duration::from_secs(0);};
    let pos = sink.get_pos();
    LOCKED = false;
    pos
}
