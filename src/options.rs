use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt(about = "Todo-list management with the command-line interface written in rust")]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// List all tasks
    List,
    /// Add a new task
    Add {
        /// Description for the task
        description: String
    },
    /// Update a task
    Update {
        /// Identifier for the task to update
        id: i16,
        /// Description for the task
        description: String
    },
    /// Delete a task
    Delete {
        /// Identifier for the task to delete
        id: i16
    },
    /// Mark a task as completed
    Complete {
        /// Identifier for the task to complete
        id: i16
    },
    /// Clear all task
    Clear
}
