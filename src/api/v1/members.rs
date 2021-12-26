use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use worker::{Error, Headers, Request, Response, RouteContext};

use crate::utils::make_github_auth_header;

#[derive(Serialize, Deserialize)]
pub struct MemberListItem {
    login: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Member {
    login: String,
    avatar_url: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub async fn find_all(mut _req: Request, ctx: RouteContext<()>) -> Result<Response, Error> {
    let auth_header = make_github_auth_header(&ctx);
    let http_client = Client::new();
    let mut all_members: Vec<Member> = Vec::new();

    // Retrieve a list of public organization members
    let public_members_list = http_client
        .get("https://api.github.com/orgs/TheComputersClub/members")
        .header("Authorization", &auth_header)
        .header("User-Agent", "reqwest v0.11.8")
        .send()
        .await
        .map_err(|err| Error::from(err.to_string()))?
        .json::<Vec<MemberListItem>>()
        .await
        .map_err(|err| Error::from(err.to_string()))?;

    // For each public organization member, fetch details using the `users`
    // GitHub API endpoint
    for member in public_members_list.iter() {
        let member_data = http_client
            .get(format!(
                "{}/{}",
                "https://api.github.com/users", member.login
            ))
            .header("Authorization", &auth_header)
            .header("User-Agent", "reqwest v0.11.8")
            .send()
            .await
            .map_err(|err| Error::from(err.to_string()))?
            .json::<Member>()
            .await
            .map_err(|err| Error::from(err.to_string()))?;

        all_members.push(member_data);
    }

    let mut response_headers = Headers::new();

    response_headers.append("Cache-Control", "max-age=86400")?;

    Ok(Response::from_json(&all_members)?.with_headers(response_headers))
}
