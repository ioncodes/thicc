#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate sqlite;
extern crate askama;
extern crate base64;

mod server;
mod db;
mod constants;
mod create_template;
mod paste_template;
mod paste;

use crate::server::Server;

fn main() {
    let server = Server::new();
    server.start();
}