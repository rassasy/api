#[derive(Debug)]
pub struct Restaurant {
    pub id : String,
    pub name : String,
    pub city : String,
    pub state : String,
    pub featurers : Vec<Featurer>,
    pub description : String,
    pub notes : String,
    pub street_addresses : Vec<String>,
    pub country : String,
    pub visited : bool,
    pub tags : Vec<Tag>,
    pub website: String,
    pub yelp: String
}

impl Restaurant {
    pub fn from(details : RestaurantDetail, relations : Restaurant) -> Restaurant {
        Restaurant {
            id: details.id,
            name: details.name,
            city: details.city,
            state: details.state,
            featurers: relations.featurers,
            description: details.description,
            notes: details.notes,
            street_addresses: relations.street_addresses,
            country: details.country,
            visited: details.visited.to_lowercase().parse::<bool>().unwrap(),
            tags: relations.tags,
            website: details.website,
            yelp: details.yelp
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
    pub notes : String,
    pub country : String,
    pub visited : String,
    pub website: String,
    pub yelp: String
}

#[derive(Debug)]
pub struct RestaurantLinks {
    pub featurers : Vec<Featurer>,
    pub street_addresses : Vec<String>,
    pub tags : Vec<Tag>
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