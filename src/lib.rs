/**
How to use :
    #[tokio::main]
    async fn main() {
        let open_api_settings = Settings::new("my_secret_key", None);
        let gpt_req = Completion::new(open_api_settings)

        let response = gpt_req.execute().await?;

    }
 */

mod models;
mod requests;
pub mod settings;
pub mod client;
pub mod errors;