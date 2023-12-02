mod args;

use std::intrinsics::likely;

use args::TodoArgs;
use clap::Parser;

fn main() {
    let args: TodoArgs = TodoArgs::parse();
}
