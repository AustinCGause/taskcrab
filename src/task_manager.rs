// use std::error::Error;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::Path};
// use fastrand::u32;
use crate::{cli::ViewType, format_helpers::output_tasks};

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks { tasks: Vec::new() }
    }

    pub fn load_from_file(file_path: &Path) -> Result<Tasks, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let tasks = serde_json::from_reader(file)?;
        Ok(tasks)
    }

    pub fn add_task( &mut self, file: File, desc: String, due: String ) -> Result<(), Box<dyn Error>> {
        self.tasks.push(Task::new(desc, due));
        self.view_tasks(ViewType::All)?;

        // Change to_writer_pretty to to_writer in final build
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn view_tasks(&self, view_type: ViewType) -> Result<(), Box<dyn Error>> {
        let tasks: Vec<Task> = match view_type {
            ViewType::All => {
                 self.tasks.clone()
            }
            ViewType::InProgress => {
                self.tasks.iter().filter(|task| !task.complete).cloned().collect()
            }
            ViewType::Completed => {
                self.tasks.iter().filter(|task| task.complete).cloned().collect()
            }
        };

        output_tasks(tasks);
        Ok(())
    }

    pub fn complete_task(&mut self, file: File, index: u32) -> Result<(), Box<dyn Error>> {
        self.tasks[index as usize].complete = true;
        // Change to_writer_pretty to to_writer in final build
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn delete_task(&mut self, file: File, index: u32) -> Result<(), Box<dyn Error>> {
        self.tasks.remove(index as usize);
        // Change to_writer_pretty to to_writer in final build
        serde_json::to_writer_pretty(file, self)?;
        self.view_tasks(ViewType::All)?;
        Ok(())
    }


    // ################################################################################
    // TEST METHOD - REMOVE IN FINAL BUILD
    pub fn clear_tasks(&mut self, file: File) -> Result<(), Box<dyn Error>> {
        self.tasks.clear();
        serde_json::to_writer(file, self)?;
        Ok(())
    }
    // ################################################################################
}

// TODO: Look into a method that doesn't involve cloning
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub desc: String,
    pub due: String,
    pub complete: bool,
    // id: u32,
}

impl Task {
    pub fn new(desc: String, due: String) -> Self {
        Task {
            desc,
            due,
            complete: false,
            // id: generate_id(),
        }
    }
}

// fn generate_id() -> u32 {
//
//     u32(99..)
// }
