#![allow(unused_imports)]
use std::io;
use std::io::Write;
use std::fmt::Write as fmtWrite;
use std::error::Error;
use rand::Rng;
use std::string::String;
use std::thread;
use std::time;
extern crate termion;
use termion::{color, style};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn nice_hex_u8(x: u8, output: &mut String){
	if x < 16 {
		write!(output, "0").unwrap();
	}
	write!(output, "{:X}", x).unwrap();
}

fn arr_to_hex(arr: &[u8]) -> String{
	let mut res = String::with_capacity(arr.len()*2);
	for x in arr.iter(){
		&nice_hex_u8(*x, &mut res);
	}
	debug_assert!(res.capacity() == arr.len()*2, "string capacity is {}, expected {}", res.capacity(), arr.len()*2);
	res
}

fn main() -> Result <(),Box<dyn Error>> {
	// Set terminal to raw mode to allow reading stdin one key at a time
	let stdout = io::stdout().into_raw_mode().unwrap();
	// Use asynchronous stdin
	let mut stdin = termion::async_stdin().keys();

	println!("hit q to exit");
	stdout.lock().flush().unwrap();

	let mut arr: [u8;3] = [0,0,0];
	for i in 0..3 {
		arr[i] = rand::thread_rng().gen_range(0, 255);
	}

	'outer: loop{
		for i in 0..3 {
			loop {
				arr[i] = rand::thread_rng().gen_range(0, 255);

				print!("\r{}#{}", termion::color::Fg(termion::color::Rgb(arr[0], arr[1], arr[2])), arr_to_hex(&arr));
				stdout.lock().flush().unwrap();

				let input = stdin.next();
				if let Some(Ok(key)) = input {
					match key {
						termion::event::Key::Char('Q') => break 'outer,
						termion::event::Key::Char('q') => break 'outer,
						_ => {
								break;
						}
					}
				}

			}
			thread::sleep(time::Duration::from_millis(100));
		}
		println!("\r{}#{}", termion::color::Fg(termion::color::Rgb(arr[0], arr[1], arr[2])), arr_to_hex(&arr));
	}

	println!("\r");
	Ok(())
}
