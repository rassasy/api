use crate::db::connection::{Databases, MySQLConnection, Neo4jConnection};
use crate::graphql::schema::{QueryRoot, MutationRoot};
use crate::models::{Restaurant, LinkNode};
use juniper::RootNode;
use rocket::{get, post};
use rocket::response::content;
use rocket::response::status;
use rocket::State;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[get("/")]
pub fn index(graph: Neo4jConnection) -> status::Accepted<String> {

    let result = graph.exec("MATCH (r:Restaurant{id: \"4321\"})-[f:FEATURED_BY]-(a) RETURN a").unwrap();

    let nodes : Vec<LinkNode> = result.rows().map(|entry| entry.get::<LinkNode>("a").unwrap()).collect();

    status::Accepted(Some(String::from("test")))
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
