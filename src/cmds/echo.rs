// use clap::ArgMatches;
use std::error;

use crate::lib::TodoList;

impl TodoList {
	pub fn echo(&self) -> Result<String, Box<dyn error::Error>> {
		println!("{:?}", self);

		Ok("Echoed Input".into())
	}
}
