// NOTE the goal of this project is to write a tool that can be used to analyze and transcribe music easily, using spectrograms
// TODO base features: visualize audio file as spectrogram, playback file, play selected bins only
// TODO feature ideas: brush-like tools for editing audio, non-destructive modifiers, instrument fingerprinting to try and separate instruments by subtracting them from the mix
// NOTE phase is important, I probably should try not to forget it

use std::{fs::File, io::BufReader, process::exit};

use rodio::{self, Decoder, OutputStream, Sink, Source};

// use dasp::signal::{interpolate::Converter, sine};

use rand::random;

mod app;

fn main() {

    const FILE_PATH: &str = "Field of Hopes and Dreams.mp3";
    // const FILE_PATH: &str = "Rude Buster.mp3";
    // const FILE_PATH: &str = "Restoring the Light, Facing the Dark.mp3";

    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to get default output stream");
    let file = BufReader::new(File::open(FILE_PATH).expect("Failed to open file"));

    let source = Decoder::new(file)
        .expect("Failed to decode file")
        .buffered();
    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

    let data = source.clone().convert_samples::<f32>().collect::<Vec<_>>();
    println!("Collected {} samples", data.len());

    sink.append(source);
    sink.set_volume(0.5);
    sink.play();

    let app = app::MusicApp::new(data);
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);
    println!("Window closed");

    // NOTE attempting to stop the sink cleanly actually causes a stackoverflow
    // NOTE exiting the entire process cleans up better on Windows
    exit(0);
}
