use rodio::OutputStream;
use minisynth::track::Track;
use minisynth::parser::parser;
use minisynth::keymap::Keymap;
use std::env;

fn stream(mut tracks: Vec<Track>) {
	for t in 0..tracks.len() {
	    tracks[t].load_sink();
	}
	for t in 0..tracks.len() {
	    tracks[t].play();
	}
	for t in 0..tracks.len() {
	    tracks[t].sleep_until_end();
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2
	{
	    println!("Usage: ./minisynth file");
	    std::process::exit(1);
	}
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let keymap = Keymap::new("assets/notes_freq_lower.json");
	let desc: Vec<String> = parser::get_desc(&args[1]);
	let tempo = parser::get_tempo(&desc);
	let mut tracks = parser::get_tracks(&desc, &stream_handle);
	parser::set_notes(&mut tracks, &desc, tempo, keymap.get());
	println!("And begin!");
	stream(tracks);
}
