use reqwest::Client;
use worker::{Error, Request, Response, RouteContext};

use crate::utils::map_reqwest_response;

pub async fn find_all(mut req: Request, ctx: RouteContext<()>) -> Result<Response, Error> {
    let github_username = ctx
        .var("GITHUB_USERNAME")
        .expect("Missing \"GITHUB_USERNAME\" environment variable");
    let github_personal_access_token = ctx
        .var("GITHUB_API_TOKEN")
        .expect("Missing \"GITHUB_API_TOKEN\" environment variable");
    let http_client = Client::new();
    let response = http_client
        .get("https://api.github.com/orgs/TheComputersClub/members")
        .basic_auth(
            github_username.to_string(),
            Some(github_personal_access_token.to_string()),
        )
        .send()
        .await
        .map_err(|err| Error::from(err.to_string()))?;

    map_reqwest_response(response).await
}
