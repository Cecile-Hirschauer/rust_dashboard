use serde::{ Deserialize, Serialize };
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Person {
    pub uuid: String,
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "level is required"))]
    pub level: String,
    #[validate(range(min = 2000, max = 99999))]
    pub compensation: u32,
    pub joined_date: String,
}

impl Person {
    pub fn new(
        uuid: String,
        name: String,
        title: String,
        level: String,
        compensation: u32,
        joined_date: String
    ) -> Person {
        Person {
            uuid,
            name,
            title,
            level,
            compensation,
            joined_date,
        }
    }
}
