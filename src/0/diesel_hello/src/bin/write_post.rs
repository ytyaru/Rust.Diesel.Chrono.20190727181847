extern crate diesel_hello;
extern crate diesel;

use self::diesel_hello::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("タイトル> ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\n本文（終了:{})\n", EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post_id = create_post(&connection, title, &body);
    println!("\n保存した。ドラフト「{}」 id: {}", title, post_id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

