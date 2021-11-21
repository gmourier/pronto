extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate prettytable;
use prettytable::{Table, format};
use clap::{Arg, App, SubCommand};
use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use console::Emoji;
extern crate chrono;
use chrono::prelude::*;

#[derive(Deserialize, Serialize)]
struct Task {
    desc: String,
    done: bool
}

fn read_tasks_data() -> Result<HashMap<i64, Task>, Box<dyn Error>> {
    let file = fs::File::open("data.json").expect("File should open read only.");
    let tasks: HashMap<i64, Task> = serde_json::from_reader(file).expect("JSON was not well-formatted.");
    Ok(tasks)
}

fn write_tasks_data(tasks: &HashMap<i64, Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string(&tasks)?;
    fs::write("data.json", &data).expect("Unable to write data to file.");
    Ok(())
}

fn print_tasks(tasks: &HashMap<i64, Task>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(row!["id", "status", "desc", "created_at"]); //table head

    for (k ,v) in tasks {
        let icon;
        if v.done {
            icon = Emoji("✅", "done");
        }
        else {
            icon = Emoji("❌", "todo");
        }

        let naive = NaiveDateTime::from_timestamp(*k, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let created_at = datetime.format("%Y-%m-%d %H:%M:%S");

        table.add_row(row![k, icon, v.desc, created_at]);
    }

    table.printstd();
}

fn main() {
    let mut tasks = read_tasks_data().unwrap();

    let matches = App::new("Pronto")
        .about("Manage your todo-list in your terminal")
        .version("v0.1.0")
        .author("Guillaume Mourier (@gmourier on GitHub)")
        .subcommand(SubCommand::with_name("list")
            .about("List tasks to-do")
        )
        .subcommand(SubCommand::with_name("add")
            .about("Add a task")
            .arg(Arg::with_name("desc")
                .help("Sets the description for the new task")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("complete")
            .about("Mark a task as completed")
            .arg(Arg::with_name("id")
                .help("Identifier marking the task to be completed")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("delete")
            .about("Delete a task")
            .arg(Arg::with_name("id")
                .help("Identifier marking the task to be deleted")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("clear")
            .about("Clear tasks list")
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        print_tasks(&tasks);
    }

    if let Some(add) = matches.subcommand_matches("add") {
        let desc: String = add.value_of("desc").unwrap().into();

        tasks.insert(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, Task { desc: desc, done: false });

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }

    if let Some(complete) = matches.subcommand_matches("complete") {
        let id: i64 = complete.value_of("id").unwrap().parse::<i64>().expect("can't parse the identifier");

        if let Some(task) = tasks.get_mut(&id) {
            task.done = true;
        }

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }

    if let Some(delete) = matches.subcommand_matches("delete") {
        let id: i64 = delete.value_of("id").unwrap().parse::<i64>().expect("can't parse the identifier");

        tasks.remove(&id);

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }

    if let Some(_) = matches.subcommand_matches("clear") {
        tasks.clear();

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }
}