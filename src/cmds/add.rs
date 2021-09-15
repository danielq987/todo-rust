use clap::ArgMatches;
use std::error;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::lib::{Todo, TodoList, PATH};

impl TodoList {
	pub fn add(&mut self, args: &ArgMatches) -> Result<String, Box<dyn error::Error>> {
		let input = args.value_of("add_input").unwrap_or("asdf");
		let todo = Todo::new(String::from(input), 1)?;

		let mut file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(PATH)
			.unwrap();

		if let Err(e) = writeln!(file, "{}", input) {
			eprintln!("Couldn't write to file: {}", e);
		}

		println!("{:?}", todo);
		self.todos.push(todo);
		println!("{:?}", self);
		Ok("Added".into())
	}
}
