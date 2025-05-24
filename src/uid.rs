use std::{
    fs::File,
    io::{Read, Write},
};

use regex::Regex;

use crate::error::Loggable;

pub struct Todo {
    todo: String,
    uid: UID,
}

pub struct UID {
    references: Vec<TodoRef>,
    hash: String,
}

pub struct TodoRef {
    line: usize,
    file: String,
}
fn line_number(text: &str, offset: usize) -> usize {
    text[..offset].bytes().filter(|&b| b == b'\n').count() + 1
}

impl TodoRef {
    pub fn from_uid(filename: &str) -> Vec<Self> {
        let mut file = File::open(filename).expect("Unreachable");
        let mut buf = String::new();
        file.read_to_string(&mut buf).log_err();
        let regex = Regex::new(r"(?m)//\s*(TODO|FIXME)\s*(\[.*])?\s*:\s*.+$").unwrap();

        for todo in regex.find_iter(&buf) {
            println!("[{}]{}", line_number(&buf, todo.start()), todo.as_str());
        }
        vec![]
    }
}

#[test]
fn test_todoref() {
    println!("test");
    let _ = File::create_new("test.file").inspect(|mut f| {
        writeln!(f, "//TODO [uid]: test\n").unwrap();
    });
    let _ = TodoRef::from_uid("test.file");
}
