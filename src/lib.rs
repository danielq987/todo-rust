use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::{error, fs};
use tabular::Row;

pub const PATH: &str = "todo.txt";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub path: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
	fn default() -> Self {
		Self {
			path: "~/.todo-rust/todo.txt".into(),
		}
	}
}

#[derive(Debug)]
pub struct TodoList {
	pub todos: Vec<Todo>,
}

impl TodoList {
	// pub fn new(todos: Vec<Todo>) -> TodoList {
	// 	TodoList { todos }
	// }
	// Reads from a file and
	pub fn from_file(filename: &str) -> Result<TodoList, Box<dyn error::Error>> {
		let contents: String = fs::read_to_string(filename)?;
		let mut parsed_todos: Vec<Todo> = Vec::new();

		for (i, line) in contents.split('\n').enumerate() {
			match Todo::new(line.into(), u32::try_from(i + 1)?) {
				Ok(todo) => parsed_todos.push(todo),
				Err(_) => (),
			}
		}

		return Ok(TodoList {
			todos: parsed_todos,
		});
	}
}

#[derive(Debug)]
pub struct Todo {
	id: u32,
	priority: Option<char>,
	description: String,
	project: Vec<String>,
	context: Vec<String>,
	is_completed: bool,
	creation_date: Option<u32>,
	completion_date: Option<u32>,
	due: u32,
}

impl Todo {
	pub fn new(_line: String, id: u32) -> Result<Todo, String> {
		const GENERIC_ERROR: &str = "Something went wrong parsing the todos";
		let mut line: Vec<&str> = _line.trim().split_whitespace().collect();

		if line.len() < 1 {
			return Err(GENERIC_ERROR.into());
		}

		let is_completed = if line[0] == "x" {
			line.remove(0);
			true
		} else {
			false
		};

		let pri_re = Regex::new(r"\([A-Z]\)").unwrap();

		let priority = if pri_re.is_match(line[0]) {
			let pri = line[0].chars().nth(1).unwrap();
			line.remove(0);
			Some(pri)
		} else {
			None
		};

		let mut contexts = Vec::new();
		let mut projects = Vec::new();

		let context_re = Regex::new(r"^@").unwrap();
		let project_re = Regex::new(r"^\+").unwrap();

		for word in line.clone() {
			if context_re.is_match(word) {
				contexts.push(word.to_owned());
			} else if project_re.is_match(word) {
				projects.push(word.to_owned());
			}
		}

		let todo = Todo {
			id,
			priority,
			description: line[..].join(" "),
			project: projects,
			context: contexts,
			creation_date: None,
			completion_date: None,
			is_completed,
			due: 123,
		};
		Ok(todo)
	}

	fn to_string(&self) -> Result<String, String> {
		Ok("".into())
	}

	pub fn to_row(&self) -> Result<Row, String> {
		let row = Row::new()
			// Id Col
			.with_cell(if self.is_completed {
				"x".to_string()
			} else {
				self.id.to_string()
			})
			// Priority Col
			.with_cell(if let None = self.priority {
				"".to_string()
			} else {
				format!("({})", self.priority.unwrap().to_string())
			})
			// Description Col
			.with_cell(self.description.clone());

		Ok(row)
	}
}
