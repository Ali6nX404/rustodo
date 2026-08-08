use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use colored::Colorize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn add(&mut self, description: String) {
        let max_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        let new_task = Task {
            id: max_id,
            description: description,
            done: false,
        };

        self.tasks.push(new_task);
    }

    pub fn list(&self) {
    for task in &self.tasks {
        if task.done {
            println!("{} {}", "[x]".green(), format!("{}: {}", task.id, task.description).dimmed());
        } else {
            println!("{} {}: {}", "[ ]".white(), task.id.to_string().yellow(), task.description);
        }
    }
}

    pub fn complete(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                if task.done != true {
                    task.done = true;
                } else {
                    println!("Task {} is already marked completed", id);
                }
            }
        }
    }

    pub fn delete(&mut self, delete_id: u32) {
        self.tasks.retain(|task| task.id != delete_id);
    }

    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let pretty_json =
            serde_json::to_string_pretty(&self).context("Failed to serialize tasks to JSON")?;
        fs::write(path, pretty_json).context("Failed to write tasks file")?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<TaskList> {
        let contents = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(TaskList { tasks: Vec::new() });
            }
            Err(e) => return Err(e).context("failed to read tasks file"),
        };

        let tasklist = serde_json::from_str(&contents)
            .context("failed to parse tasks file — JSON may be corrupted")?;
        Ok(tasklist)
    }
}