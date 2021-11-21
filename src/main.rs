mod options;

use serde::{ Serialize, Deserialize };
use prettytable::{ Table, Row, Cell, format };
use std::collections::HashMap;
use std::error::Error;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::fs;
use console::Emoji;
use chrono::prelude::*;

use options::{ Command, Options };
use structopt::StructOpt;

#[derive(Deserialize, Serialize)]
struct Task {
    description: String,
    completed: bool,
    created_at: i64
}

fn read_tasks_data() -> Result<HashMap<i16, Task>, Box<dyn Error>> {
    let file = fs::File::open("data.json").expect("File should open read only");
    let tasks: HashMap<i16, Task> = serde_json::from_reader(file).expect("JSON was not well-formatted");
    Ok(tasks)
}

fn write_tasks_data(tasks: &HashMap<i16, Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string(&tasks)?;
    fs::write("data.json", &data).expect("Unable to write data to file");
    Ok(())
}

fn print_tasks(tasks: &HashMap<i16, Task>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![
        Cell::new("id"),
        Cell::new("status"),
        Cell::new("desc"),
        Cell::new("created_at")
    ]));

    for (k ,v) in tasks {
        let icon;
        if v.completed {
            icon = Emoji("✅", "done");
        }
        else {
            icon = Emoji("❌", "todo");
        }

        let naive = NaiveDateTime::from_timestamp(v.created_at, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let created_at = datetime.format("%Y-%m-%d %H:%M:%S");

        table.add_row(Row::new(vec![
            Cell::new(&k.to_string()),
            Cell::new(&icon.to_string()),
            Cell::new(&v.description),
            Cell::new(&created_at.to_string())
        ]));
    }

    table.printstd();
}

fn add(tasks: &mut HashMap<i16, Task>, description: String) {
    //set current auto-increment for task id
    let mut c: i16 = 0;
    if let Some(&max) = tasks.keys().max(){
        c = max;
    }

    tasks.insert(
        c + 1,
        Task {
            description: description,
            completed: false,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    );
}

fn update(tasks: &mut HashMap<i16, Task>, id: i16, description: String) {
    if let Some(task) = tasks.get_mut(&id) {
        task.description = description;
    }
}

fn complete(tasks: &mut HashMap<i16, Task>, id: i16) {
    if let Some(task) = tasks.get_mut(&id) {
        task.completed = true;
    }
}

fn delete(tasks: &mut HashMap<i16, Task>, id: i16) {
    tasks.remove(&id);
}

fn clear(tasks: &mut HashMap<i16, Task>) {
    tasks.clear();
}

fn main() {
    //Load tasks from file at startup
    let mut tasks = read_tasks_data().unwrap();

    let opt = Options::from_args();
    match opt.command {
        Command::List => {
            print_tasks(&tasks);
        },
        Command::Add {
            description
        } => {
            add(&mut tasks, description);
            write_tasks_data(&tasks).unwrap_or(());
            print_tasks(&tasks);
        },
        Command::Update {
            id,
            description
        } => {
            update(&mut tasks, id, description);
            write_tasks_data(&tasks).unwrap_or(());
            print_tasks(&tasks);
        },
        Command::Delete {
            id
        } => {
            delete(&mut tasks, id);
            write_tasks_data(&tasks).unwrap_or(());
            print_tasks(&tasks);
        },
        Command::Complete {
            id
        } => {
            complete(&mut tasks, id);
            write_tasks_data(&tasks).unwrap_or(());
            print_tasks(&tasks);
        },
        Command::Clear => {
            clear(&mut tasks);
            write_tasks_data(&tasks).unwrap_or(());
            print_tasks(&tasks);
        }
    }
}