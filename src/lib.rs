#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iso8601_duration;
extern crate itertools;
extern crate juniper;
extern crate juniper_rocket;
extern crate uuid;

pub mod domain;
pub mod infrastructure;
