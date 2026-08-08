mod task;
use clap::{Parser, Subcommand};
use task::TaskList;

const DATA_FILE: &str = "tasks.json";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut task_list = TaskList::load(DATA_FILE)?;

    match cli.command {
        Commands::Add { description } => {
            task_list.add(description);
            task_list.save(DATA_FILE)?;
        }
        Commands::List => {
            task_list.list();
        }
        Commands::Complete { id } => {
            task_list.complete(id);
            task_list.save(DATA_FILE)?;
        }
        Commands::Delete { id } => {
            task_list.delete(id);
            task_list.save(DATA_FILE)?;
        }
    }

    Ok(())
}
