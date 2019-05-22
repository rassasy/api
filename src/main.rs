#![feature(decl_macro, proc_macro_hygiene, custom_attribute)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate juniper;
#[macro_use] extern crate rocket;
extern crate rusted_cypher;

mod routes;
mod models;
mod db;
mod graphql;

fn main() {
    rocket::ignite()
        .manage(routes::Schema::new(
            graphql::schema::QueryRoot,
            graphql::schema::MutationRoot,
        ))
        .mount("/", routes![
            routes::index,
            routes::graphiql,
            routes::get_graphql_handler,
            routes::post_graphql_handler
        ])
        .attach(db::connection::Neo4jConnection::fairing())
        .attach(db::connection::MySQLConnection::fairing())
        .launch();
}