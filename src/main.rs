//TO DO application
//usage: todo <arg>
// -a [todo] - adds todo to list
// -r [exitsting] - removes existing from list

mod args;

use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use args::ToDoArgs;
use clap::Parser;

use crate::args::EntityType;

struct ToDoElement {
    id: u32,
    name: String,
}

struct ToDoList {
    list: Vec<ToDoElement>,
}

impl ToDoElement {
    fn new(file: String, name: String) -> ToDoElement {
        let to_do_element = ToDoElement {
            id: {
                let f = BufReader::new(fs::File::open(file).unwrap());
                let mut maximum = 1;
                for line in f.lines() {
                    for ch in line.unwrap().chars() {
                        if ch.is_numeric() {
                            match ch.to_digit(10) {
                                Some(num) => {
                                    maximum = num;
                                }
                                None => {
                                    println!("cannot convert ID to number")
                                }
                            }
                        }
                    }
                }
                maximum + 1
            },
            name: String::from("Temp"),
        };
        to_do_element
    }
}

fn main() {
    let args = ToDoArgs::parse();
    let mut db = OpenOptions::new()
        .write(true)
        .append(true)
        .open("db.txt")
        .unwrap();
    let db_read = BufReader::new(fs::File::open("db.txt").unwrap());

    match args.entity_type {
        EntityType::Add(elem) => {
            //What to do with element if arg is add
            let mut todo = String::new();
            let mut id = 1;
            for _ in db_read.lines() {
                id += 1
            }
            let content = format!("{}\t{}\t", id, elem.element);
            todo.push_str(&content);
            // fs::write("db.txt", todo.as_bytes());
            writeln!(db, "{}", todo);
        }
        EntityType::Remove(elem) => {
            println!("{}", elem.element);
            if elem.element.parse().unwrap() {
                for line in db_read.lines() {}
            }
        }
        EntityType::Show => {
            println!("Show command aplied!");
            for line in db_read.lines() {
                println!("{}", line.unwrap());
            }
        }
    }
}
