use bpaf::Bpaf;
use std::fs::{self, File};
use std::io::{Error, Write};

const FILEPATH: &str = "db/todos.txt";

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, guard(add_todo_guard, "Todo can't be empty."))]
    /// Add todos
    pub add_todo: String,
}

fn add_todo_guard(input: &String) -> bool {
    !input.is_empty()
}

fn get_todos(input: &u16) {}

fn add_todo(input: String) -> std::io::Result<&'static str> {
    // check if text file exists
    if fs::metadata(FILEPATH).is_ok() {
        let mut f = File::options().append(true).open(FILEPATH)?;
        writeln!(&mut f, "{}", input)?;
        Ok("Another todo saved!")
    } else {
        fs::write(FILEPATH, input)?;
        Ok("Your first todo was successfully saved!")
    }
}

fn main() {
    let opts: Arguments = arguments().run();

    // add todo if it was given via the cli arguments
    if !opts.add_todo.is_empty() {
        let result: Result<&str, Error> = add_todo(opts.add_todo);

        match result {
            Ok(ok) => println!("{}", ok),
            Err(error) => eprintln!("{}", error),
        }
    }
}
