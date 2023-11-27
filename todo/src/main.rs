mod args;

use args::TodoArgs;
use clap::Parser;

fn main() {
    let args: TodoArgs = TodoArgs::parse();
}
