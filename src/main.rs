mod client;
mod data_model;
use client::client::OpenSkyClient;
use data_model::responses::StateAllResponse;

#[tokio::main]
async fn main() {
    let client = OpenSkyClient::new();
    let response = OpenSkyClient::states_all(&client).await;

    match response {
        Ok(body) => {
            let response: StateAllResponse = StateAllResponse::from_string(body).unwrap();
            print!("{:?}", response);
        }
        Err(e) => println!("Error: {}", e),
    }
}
