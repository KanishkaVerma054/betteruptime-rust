use std::sync::{Arc, Mutex};

//poem for routing
use poem::{
    handler, web::{Data, Json, Path}
};
use store::store::Store;

use crate::{request_inputs::{CreateWebsiteInput}, request_outputs::{CreateWebsiteOutput, GetWebsiteOutput}};

#[handler]
pub fn get_website(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    Json(GetWebsiteOutput {
        url: website.url
    })
}

#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.create_website(String::from("a01b0fb8-4335-47e1-aee9-3f818553e4d8"), data.url).unwrap();
    let response = CreateWebsiteOutput {
        id: website.id
    };
    // persist in the DB
    Json(response)
}