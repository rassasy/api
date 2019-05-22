use crate::db::connection::{Databases, MySQLConnection, Neo4jConnection};
use crate::graphql::schema::{QueryRoot, MutationRoot};
use crate::models::{Restaurant};
use juniper::RootNode;
use rocket::{get, post};
use rocket::response::content;
use rocket::response::status;
use rocket::State;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[get("/")]
pub fn index(graph: Neo4jConnection) -> status::Accepted<String> {

    let result = graph.exec("MATCH(r:Restaurant { id: \"1234\" }) RETURN r.id, r.name").unwrap();

    let restaurants : Vec<Restaurant> = result.rows().map(|entry| {
        Restaurant {
            id: entry.get("r.id").unwrap(),
            name: entry.get("r.name").unwrap(),
            featurers: vec![],
            city: String::from("Tempe"),
            state: String::from("Arizona"),
            notes: String::from("notes"),
            street_addresses: vec![],
            description: String::from("description"),
            visited: true,
            tags: vec![],
            website: String::from("www.google.com"),
            yelp: String::from("www.yelp.com"),
            country: String::from("USA")
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
    neo4j: Neo4jConnection,
    mysql: MySQLConnection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Databases { neo4j, mysql })
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    neo4j: Neo4jConnection,
    mysql: MySQLConnection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Databases { neo4j, mysql })
}
