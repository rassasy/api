use rocket_contrib::database;

#[database("neo4j")]
pub struct Neo4jConnection(rusted_cypher::GraphClient);