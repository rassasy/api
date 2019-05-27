extern crate array_tool;

use array_tool::vec::Union;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Restaurant {
    pub id : String,
    pub name : String,
    pub city : String,
    pub state : String,
    pub featurers : Vec<Featurer>,
    pub description : String,
    pub notes : Option<String>,
    pub street_addresses : Vec<String>,
    pub country : String,
    pub visited : bool,
    pub tags : Vec<Tag>,
    pub website: Option<String>,
    pub yelp: String
}

impl Restaurant {
    pub fn from(details : RestaurantDetail) -> Restaurant {
        Restaurant {
            id: details.id,
            name: details.name,
            city: details.city,
            state: details.state,
            featurers: Vec::new(),
            description: details.description,
            notes: details.notes,
            street_addresses: Vec::new(),
            country: details.country,
            visited: details.visited.to_lowercase().parse::<bool>().unwrap(),
            tags: Vec::new(),
            website: details.website,
            yelp: details.yelp
        }
    }

    pub fn with_links(self, links : RestaurantLinks) -> Restaurant {
        Restaurant {
            id: self.id,
            name: self.name,
            city: self.city,
            state: self.state,
            featurers: links.featurers,
            description: self.description,
            notes: self.notes,
            street_addresses: links.street_addresses,
            country: self.country,
            visited: self.visited,
            tags: links.tags,
            website: self.website,
            yelp: self.yelp
        }
    }
}

#[derive(Debug, Queryable)]
pub struct RestaurantDetail {
    pub id : String,
    pub name : String,
    pub city : String,
    pub state : String,
    pub description : String,
    pub notes : Option<String>,
    pub country : String,
    pub visited : String,
    pub website: Option<String>,
    pub yelp: String
}

#[derive(Debug)]
pub struct RestaurantLinks {
    pub featurers : Vec<Featurer>,
    pub street_addresses : Vec<String>,
    pub tags : Vec<Tag>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkNode {
    pub id: String
}

#[derive(Debug)]
pub struct Featurer {
    pub id : String,
    pub name : String,
    pub featurer_type : FeaturerType
}

#[derive(Debug)]
pub enum FeaturerType {
    PERSON,
    SHOW
}

#[derive(Debug)]
pub struct Tag {
    pub id : String,
    pub name : String
}