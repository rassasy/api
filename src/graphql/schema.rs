use juniper::{Context as JuniperContext, FieldResult};

use crate::models::{Restaurant};
use crate::db::{Neo4jConnection};

pub struct Context {
    pub connection: Neo4jConnection
}

impl JuniperContext for Context {}

graphql_object!(Restaurant: () |&self| {
    description: "A restaurant"

    field id() -> String as "The unique id of the restaurant" {
        self.id.clone()
    }

    field name() -> String as "The name of the restaurant" {
        self.name.clone()
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field restaurants(&executor) -> FieldResult<Vec<Restaurant>> {
        return Ok(vec![Restaurant{
                id: String::from("123455677"),
                name: String::from("haha created")
        }]);            
    }
});


pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field add_restaurant(&executor, title: String, completed: bool) -> FieldResult<Restaurant>
        as "Create a new Restaurant and return it"
    {
        return Ok(Restaurant {
            id: String::from("123455677"),
            name: String::from("haha created")
        })
    }
});

    // field update_todo(&executor, id: i32, completed: Option<bool>, title: Option<String>) -> FieldResult<Option<Todo>>
    //     as "Update an existing todo item.\
    //     \
    //     Will only updated the provided fields - if either `completed` or `title`\
    //     are omitted or null, they will be ignored.\
    //     \
    //     The mutation will return null if no todo item with the specified ID could be found."
    // {
    //     use crate::schema::todos::dsl;
    //     use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};

    //     let updated = diesel::update(dsl::todos.find(id))
    //         .set((
    //             completed.map(|completed| dsl::completed.eq(completed)),
    //             title.map(|title| dsl::title.eq(title)),
    //         ))
    //         .execute(&*executor.context().connection)?;

    //     if updated == 0 {
    //         Ok(None)
    //     } else {
    //         Ok(Some(dsl::todos.find(id)
    //             .get_result::<Todo>(&*executor.context().connection)?))
    //     }
//     // }
// });