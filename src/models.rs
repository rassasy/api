#[derive(Debug)]
pub struct Restaurant {
    pub id : String,
    pub name : String,
    pub featurers : Vec<Featurer>
}

#[derive(Debug)]
pub struct Featurer {
    pub id : String,
    pub name : String
}