use crate::schema::schema::businesses;
use serde::{Deserialize, Serialize };
use diesel::{Insertable, Queryable};
use chrono::NaiveDateTime; 

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Business{
     id: i32,
     uuid: String,
     business: String,
     business_type: String,
     location: String,
     selected_amenities: Vec<String>,
     images: Vec<String>,
     business_name: String,
     telephone_number: String,
     business_description: String,
     days_of_operation: Vec<String>,
     opening_hours: String,
     closing_hours: String,
     county: String,
     town: String,
     created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "businesses"]
pub struct NewBusiness {
    uuid: String,
    business: String,
    business_type: String,
    location: String,
    selected_amenities: Vec<Option<String>>,
    images: Vec<Option<String>>,
    business_name: String,
    telephone_number: String,
    business_description: String,
    days_of_operation: Vec<Option<String>>,
    opening_hours: String,
    closing_hours: String,
    county: String,
    town: String,
    created_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessPayload{
    business: String,
    business_type: String,
    location: String,
    selected_amenities: Vec<String>,
    images: Vec<String>,
    business_name: String,
    telephone_number: String,
    business_description: String,
    days_of_operation: Vec<String>,
    opening_hours: String,
    closing_hours: String,
    county: String,
    town: String,
}