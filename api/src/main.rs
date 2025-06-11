//poem for routing
use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use crate::{request_inputs::CreateWebsiteInput, request_outputs::CreateWebsiteOutput};

use store::Store;

pub mod request_inputs;
pub mod request_outputs;

// getting json inputs, giving json outputs
#[handler]
fn get_website(Path(name): Path<String>) -> String {
    
    format!("hello: {name}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s = Store{};
    let id = s.create_website();
    let response = CreateWebsiteOutput {
        id
    };
    // persist in the DB
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));
    // create and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
