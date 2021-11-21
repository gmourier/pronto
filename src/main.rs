use clap::{Arg, App, SubCommand};
use serde::{Serialize, Deserialize};
use prettytable::{Table, Row, Cell, format};
use std::collections::HashMap;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use console::Emoji;
use chrono::prelude::*;

#[derive(Deserialize, Serialize)]
struct Task {
    desc: String,
    done: bool,
    created_at: i64
}

fn read_tasks_data() -> Result<HashMap<i64, Task>, Box<dyn Error>> {
    let file = fs::File::open("data.json").expect("File should open read only");
    let tasks: HashMap<i64, Task> = serde_json::from_reader(file).expect("JSON was not well-formatted");
    Ok(tasks)
}

fn write_tasks_data(tasks: &HashMap<i64, Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string(&tasks)?;
    fs::write("data.json", &data).expect("Unable to write data to file");
    Ok(())
}

fn print_tasks(tasks: &HashMap<i64, Task>) {
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
        if v.done {
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
            Cell::new(&v.desc),
            Cell::new(&created_at.to_string())
        ]));
    }

    table.printstd();
}

fn main() {
    let mut tasks = read_tasks_data().unwrap();

    //set current auto-increment for task id
    let mut c: i64 = 0;
    if let Some(&max) = tasks.keys().max(){
        c = max;
    }

    let matches = App::new("Pronto")
        .about("Manage your todo-list in your terminal")
        .version("v0.1.0")
        .author("Guillaume Mourier (@gmourier on GitHub)")
        .subcommand(SubCommand::with_name("l")
            .about("List tasks to-do")
        )
        .subcommand(SubCommand::with_name("a")
            .about("Add a task")
            .arg(Arg::with_name("desc")
                .help("Sets the description for the new task")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("r")
            .about("Rename a task")
            .arg(Arg::with_name("id")
                .help("Identifer for the task to be updated")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("desc")
                .help("Sets the description for a task")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name("c")
            .about("Mark a task as completed")
            .arg(Arg::with_name("id")
                .help("Identifier marking the task to be completed")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("d")
            .about("Delete a task")
            .arg(Arg::with_name("id")
                .help("Identifier marking the task to be deleted")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("cl")
            .about("Clear tasks list")
        )
        .subcommand(SubCommand::with_name("s")
            .about("Search for a substring in a task desc")
            .arg(Arg::with_name("sub_string")
                .help("Search pattern")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("e")
            .about("Export tasks list to a the specified format")
            .arg(Arg::with_name("format")
                .help("Format to export")
                .required(true)
            )
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("l") {
        print_tasks(&tasks);
    }

    if let Some(add) = matches.subcommand_matches("a") {
        let desc: String = add.value_of("desc").unwrap().into();

        tasks.insert(
            c + 1,
            Task {
                desc: desc,
                done: false,
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
            }
        );

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }

    if let Some(rename) = matches.subcommand_matches("r") {
        let id: i64 = rename.value_of("id").unwrap().parse::<i64>().expect("can't parse the identifier");
        let desc: String = rename.value_of("desc").unwrap().into();

        if let Some(task) = tasks.get_mut(&id) {
            task.desc = desc;
        }

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }

    if let Some(complete) = matches.subcommand_matches("c") {
        let id: i64 = complete.value_of("id").unwrap().parse::<i64>().expect("can't parse the identifier");

        if let Some(task) = tasks.get_mut(&id) {
            task.done = true;
        }

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }

    if let Some(delete) = matches.subcommand_matches("d") {
        let id: i64 = delete.value_of("id").unwrap().parse::<i64>().expect("can't parse the identifier");

        tasks.remove(&id);

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }

    if let Some(_) = matches.subcommand_matches("cl") {
        tasks.clear();

        write_tasks_data(&tasks).unwrap_or(());
        print_tasks(&tasks);
    }
}