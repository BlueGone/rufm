extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate rufm_core;

embed_migrations!("migrations");
