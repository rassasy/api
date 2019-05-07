#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;
extern crate rusted_cypher;

mod models;
mod graphql;
mod db;

use crate::db::Neo4jConnection;
use crate::models::{Restaurant};
use crate::graphql::schema::{QueryRoot, MutationRoot, Context};
use juniper::RootNode;
use rocket::response::content;
use rocket::response::status;
use rocket::State;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[get("/")]
fn index(graph: Neo4jConnection) -> status::Accepted<String> {

    let result = graph.exec("MATCH(r:Restaurant { id: \"1234\" }) RETURN r.id, r.name").unwrap();

    let restaurants : Vec<Restaurant> = result.rows().map(|entry| {
        Restaurant {
            id: entry.get("r.id").unwrap(),
            name: entry.get("r.name").unwrap(),
            featurers: vec![]
        }
    }).collect();

    status::Accepted(Some(format!("{:?}", restaurants)))
}

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: Neo4jConnection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: Neo4jConnection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: context })
}

fn main() {
    rocket::ignite()
        .manage(Schema::new(
            QueryRoot,
            MutationRoot,
        ))
        .mount("/", routes![index, graphiql, get_graphql_handler, post_graphql_handler])
        .attach(Neo4jConnection::fairing())
        .launch();
}