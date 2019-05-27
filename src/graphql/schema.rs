use juniper::{Context as JuniperContext, FieldResult, LookAheadMethods};

use crate::db::diesel::client::restaurant as RestaurantClient;
use crate::db::connection::Databases;
use crate::models::{Featurer, FeaturerType, Restaurant, RestaurantDetail, RestaurantLinks};

impl JuniperContext for Databases {}

graphql_object!(Restaurant: () |&self| {
    description: "A restaurant"

    field id() -> String as "The unique id of the restaurant" {
        self.id.clone()
    }

    field name() -> String as "The name of the restaurant" {
        self.name.clone()
    }

    field featuredBy() -> &Vec<Featurer> as "The featurers" {
        &self.featurers
    }
});

graphql_object!(Featurer: () |&self| {
    description: "A featurer"

    field id() -> String as "The unique id of the restaurant" {
        self.id.clone()
    }

    field name() -> String as "The name of the restaurant" {
        self.name.clone()
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Databases |&self| {

    field restaurants(&executor) -> FieldResult<Vec<Restaurant>> {
        let look_ahead = executor.look_ahead();

        let details : Vec<RestaurantDetail> = RestaurantClient::list(&*executor.context().mysql).unwrap();

        if vec!["featuredBy", "taggedWith", "locatedAt"].into_iter().any(|field| look_ahead.has_child(field)) {
            println!("true")
        }

        let restaurants : Vec<Restaurant> = details.into_iter().map(|detail| Restaurant::from(detail)).collect();

        //TODO: finish this implementation
        return Ok(restaurants);

    }

    field restaurant(&executor, id: String) -> FieldResult<Restaurant> {
        let look_ahead = executor.look_ahead();

        let details = RestaurantClient::get(&*executor.context().mysql, id).unwrap();

        if vec!["featuredBy", "taggedWith", "locatedAt"].into_iter().any(|field| look_ahead.has_child(field)) {
            let links = RestaurantLinks { featurers: vec![], tags: vec![], street_addresses: vec![] };

            return Ok(Restaurant::from(details).with_links(links))
        }

        return Ok(Restaurant::from(details));
    }
});


pub struct MutationRoot;

graphql_object!(MutationRoot: Databases |&self| {
    field add_restaurant(&executor, title: String, completed: bool) -> FieldResult<Restaurant>
        as "Create a new Restaurant and return it"
    {
        return Ok(Restaurant {
            id: String::from("1234"),
            name: String::from("Nate's"),
            featurers: vec![],
            city: String::from("Tempe"),
            state: String::from("Arizona"),
            notes: Some(String::from("notes")),
            street_addresses: vec![],
            description: String::from("description"),
            visited: true,
            tags: vec![],
            website: Some(String::from("www.google.com")),
            yelp: String::from("www.yelp.com"),
            country: String::from("USA")
        });
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