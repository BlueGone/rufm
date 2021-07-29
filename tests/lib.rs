extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate rufm;

embed_migrations!("migrations");
