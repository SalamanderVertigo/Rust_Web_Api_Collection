use serde::{Deserialize, Serialize} 


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

impl User {
    pub fn new() -> User {
        let new_user = User {
            id: 787,
            first_name: "Jon",
            last_name: "Gucciardi",
            email: "Jon@email.com",   
        };
        new_user
    }
}