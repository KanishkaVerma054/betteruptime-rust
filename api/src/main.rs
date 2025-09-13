//poem for routing
use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use crate::{request_inputs::{CreateUserInput, CreateWebsiteInput}, request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};

use store::{store::Store};

pub mod request_inputs;
pub mod request_outputs;

// getting json inputs, giving json outputs
#[handler]
fn get_website(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::new().unwrap();
    let website = s.get_website(id).unwrap();
    Json(GetWebsiteOutput {
        url: website.url
    })
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s = Store::new().unwrap();
    let id = s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput {
        id
    };

    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SigninOutput> {
    let mut s = Store::new().unwrap();
    let _exists = s.sign_in(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: String::from("kanishk")
    };

    Json(response)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s = Store::new().unwrap();
    let website = s.create_website(String::from("a01b0fb8-4335-47e1-aee9-3f818553e4d8"), data.url).unwrap();
    let response = CreateWebsiteOutput {
        id: website.id
    };
    // persist in the DB
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in));
    // create and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
