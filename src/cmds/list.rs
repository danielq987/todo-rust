// use clap::ArgMatches;
use std::error;
use tabular::Table;

use crate::lib::TodoList;

impl TodoList {
	pub fn list(&self) -> Result<String, Box<dyn error::Error>> {
		let mut table = Table::new("{:<}  {:<}  {:<}");

		for i in &self.todos {
			table.add_row(i.to_row()?);
		}

		println!("{}", table);

		Ok("Listed todo".into())
	}
}
