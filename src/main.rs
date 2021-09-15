use clap::{load_yaml, App};
use std::error;

mod cmds;
mod lib;

use lib::TodoList;

fn main() {
    match run() {
        Err(err) => println!("{:?}", err),
        Ok(_) => (),
    };
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // TODO - Use Config File instead of hard-coded path.
    let mut todo_list = TodoList::from_file(lib::PATH)?;

    println!("{:?}", todo_list);

    let result: Result<String, Box<dyn error::Error>> = match matches.subcommand() {
        ("add", Some(x)) => todo_list.add(x),
        ("echo", _) => todo_list.echo(),
        ("ls", _) => todo_list.list(),
        _ => Err("Not a command!".into()),
    };

    match result {
        Ok(x) => println!("{}", x),
        Err(e) => println!("Error: {}", e),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
