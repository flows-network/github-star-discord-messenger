use discord_flows::create_text_message_in_channel;
use dotenv::dotenv;
use github_flows::{listen_to_event, EventPayload,  GithubLogin::Default};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());

    listen_to_event(
        &Default,
        &github_owner,
        &github_repo,
        vec!["star"],
        |payload| handler(&github_repo, payload),
    )
    .await;

    Ok(())
}

async fn handler(repo: &str, payload: EventPayload) {
    let discord_server = env::var("discord_server").unwrap_or("myserver".to_string());
    let discord_channel = env::var("discord_channel").unwrap_or("general".to_string());

    if let EventPayload::UnknownEvent(e) = payload {
        let stargazers_count = e["repository"]["stargazers_count"].as_i64().unwrap_or(-1);

        let text =
            format!("Congratulations on your repository {repo} with {stargazers_count} stars.");

        if stargazers_count % 10 == 0 {
            create_text_message_in_channel(&discord_server, &discord_channel, text, None);
        }
    }
}
