use colored::*;
use hint::RuntimeHinter;
use std::{io::Write, process::exit};

mod error;
mod files;
mod format;
mod hint;

use crate::{error::ReplError, format::contains_rolls};
use clap::Arg;
use files::FileLoader;
use ivory_runtime::{
	runtime::{Runtime, RuntimeContext},
	value::Value,
};
use rustyline::{error::ReadlineError, Editor};
struct App<'a> {
	runtime: &'a Runtime,
	loader: FileLoader,
}

impl<'a> App<'a> {
	fn run(&mut self, cmd: &str) -> Result<(), ReplError> {
		let res_eq = self.runtime.run(cmd)?.un_nest();
		let res_eq_str = format!("{}", res_eq);
		let res_val: Value =
			self.runtime.math_to_value(res_eq, &RuntimeContext::new())?;
		if res_eq_str == format!("{}", res_val) {
			println!("{}", res_val);
		} else {
			println!("{} = {}", res_eq_str, res_val);
		}
		Ok(())
	}

	fn run_loop(&mut self) {
		let mut rl = Editor::<RuntimeHinter>::new();
		rl.set_helper(Some(RuntimeHinter(self.runtime)));
		loop {
			let zinger = self
				.loader
				.file_display_name()
				.unwrap_or_else(|| "ivory".to_string());
			match rl.readline(&format!("{} ~ ", zinger)) {
				Ok(line) => {
					rl.add_history_entry(&line);
					if let Err(err) = self.run(&line) {
						let err_str = format!("{}", err).red();
						println!("{}\n", err_str);
					} else {
						println!("");
					}
				}
				Err(ReadlineError::Interrupted) => {
					println!("Ctrl+C");
					break;
				}
				Err(ReadlineError::Eof) => {
					println!("Ctrl+D");
					break;
				}
				Err(err) => {
					println!("Error reading line: {}", err);
				}
			}
		}
	}
}

fn main() {
	if ansi_term::enable_ansi_support().is_err() {
		colored::control::set_override(false);
	}
	let matches = clap::App::new("Ivory")
		.version(clap::crate_version!())
		.arg(
			Arg::with_name("FILE")
				.help("A file path or url to load")
				.required(false),
		)
		.arg(
			Arg::with_name("DEBUG")
				.short("d")
				.long("debug")
				.help("Print a debug tree of the module")
				.required(false),
		)
		.arg(
			Arg::with_name("RUN")
				.short("r")
				.long("run")
				.help("Run a single command and exit")
				.takes_value(true),
		)
		.get_matches();

	let file = matches.value_of("FILE");
	let run = matches.value_of("RUN");
	let debug = matches.is_present("DEBUG");

	let mut runtime = Runtime::new(rand::thread_rng(), ());
	let mut loader = FileLoader { current_file: None };

	if let Some(file) = file {
		if debug {
			let module = loader
				.get_module(file)
				.expect("Error loading module for debug");
			println!("{:#?}", module);
			std::io::stdout().flush().unwrap();
			exit(0);
		}
		loader
			.load(&mut runtime, file)
			.expect("Unable to load file");
	}

	let mut app = App {
		runtime: &runtime,
		loader,
	};
	if let Some(run) = run {
		app.run(run).expect("error running expression");
	} else {
		app.run_loop();
	}
}
