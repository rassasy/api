use diesel::{QueryDsl, RunQueryDsl, MysqlConnection};

use crate::db::diesel::schema::restaurant_detail::dsl;
use crate::models::RestaurantDetail;

pub fn list(connection: &MysqlConnection) -> Result<Vec<RestaurantDetail>, String> {
    return dsl::restaurant_detail.order(dsl::id)
        .load::<RestaurantDetail>(connection)
        .map_err(|error| -> String { format!("{}", error)});
}

pub fn get(connection: &MysqlConnection, id: String) -> Result<RestaurantDetail, String> {
    return dsl::restaurant_detail.find(id)
        .first::<RestaurantDetail>(connection)
        .map_err(|error| -> String { format!("{}", error)});
}