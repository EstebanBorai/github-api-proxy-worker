use reqwest::Client;
use worker::{Error, Request, Response, RouteContext};

use crate::utils::{make_github_auth_header, map_reqwest_response};

pub async fn find_all(mut req: Request, ctx: RouteContext<()>) -> Result<Response, Error> {
    let auth_header = make_github_auth_header(&ctx);
    let http_client = Client::new();
    let response = http_client
        .get("https://api.github.com/orgs/TheComputersClub/members")
        .header("Authorization", auth_header)
        .header("User-Agent", "reqwest v0.11.8")
        .send()
        .await
        .map_err(|err| Error::from(err.to_string()))?;

    map_reqwest_response(response).await
}
