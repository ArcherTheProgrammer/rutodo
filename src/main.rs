//TO DO application
//usage: todo <arg>
// -a [todo] - adds todo to list
// -r [exitsting] - removes existing from list

mod args;

use std::{
    clone,
    fmt::write,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
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
    fn new(file: String, name: &std::string::String) -> ToDoElement {
        let to_do_element = ToDoElement {
            id: {
                let f = BufReader::new(fs::File::open(file).unwrap());
                let mut maximum = 0;
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
            // let mut id = 1;
            // for _ in db_read.lines() {
            //     id += 1
            // }
            let to_add = ToDoElement::new(String::from("db.txt"), &elem.element);
            let content = format!("{}\t{}\t", to_add.id, elem.element);
            todo.push_str(&content);
            // fs::write("db.txt", todo.as_bytes());
            writeln!(db, "{}", todo).unwrap();
        }
        EntityType::Remove(elem) => {
            //Writing a file to string and modifing a string, then write it to file

            let mut content: String = String::new();
            content = fs::read_to_string("db.txt").unwrap();
            let mut lines: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
            for i in 0..lines.len() - 1 {
                if lines[i].contains(&elem.id.to_string()) {
                    lines.remove(i);
                }
            }
            fs::write("db.txt", "").unwrap();
            for line in lines {
                writeln!(db, "{}", line).unwrap();
            }
            // for line in db_read.lines() {
            //     let mut temp = Vec::new();
            //     if !line.unwrap().contains(&elem.id.to_string()) {
            //         temp.push(line.as_ref());
            //     }
            //     buffer.push(temp);
            // }

            //Method below doesn't work
        }
        EntityType::Show => {
            println!("Show command aplied!");
            for line in db_read.lines() {
                println!("{}", line.unwrap());
            }
        }
    }
}
