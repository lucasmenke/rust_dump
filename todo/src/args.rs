use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show ToDo's
    Todo(ToDoCommand),
}

#[derive(Debug, Args)]
pub struct ToDoCommand {
    #[clap(subcommand)]
    pub command: ToDoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ToDoSubcommand {
    /// Create a new ToDo
    Create(CreateToDo),

    /// Update an existing ToDo
    //Update(UpdateToDo),

    /// Delete an existing ToDo
    //Delete(DeleteToDo),

    /// Show all ToDo's
    Show,
}

#[derive(Debug, Args)]
pub struct CreateToDo {
    /// The ToDo itself
    pub todo: String,

    /// The priority
    pub priority: u8,
}
