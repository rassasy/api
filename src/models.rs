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

#[derive(Debug, Queryable)]
pub struct RestaurantDetails {
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