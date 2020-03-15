use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut milis: u64  = 150;
	let filename = &args[1];
	if args.len() > 2 {
		let s_milis = &args[2];
		milis = s_milis.parse::<u64>().unwrap();
	}

	thread::sleep(Duration::from_secs(2));
	let mut enigo = Enigo::new();

	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);

	for line in reader.lines() {
		let l = line.unwrap();
	for c in l.chars(){
			enigo.key_sequence(&*c.to_string());
			thread::sleep(Duration::from_millis(milis));
		}
		enigo.key_click(Key::Raw(13));
	}
}