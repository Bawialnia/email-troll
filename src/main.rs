#![warn(clippy::pedantic)]

mod services;

use cargo::util::config::Config;
use cargo::util::{Progress, ProgressStyle};
use clap::Parser;

/// program do trollowania adresów email
#[derive(Parser)]
#[clap(help_message = "nie wiem czego nie rozumiesz")]
struct Args {
    /// email do ztrollowania
    #[clap(index(1))]
    email: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let email = Args::parse().email;

    let config = Config::default().unwrap();
    let client = reqwest::Client::new();

    let requests = services::get_requests(&client, &email, "test", "test");
    let request_count = requests.len();

    let handles = requests
        .into_iter()
        .map(|(name, request)| {
            (
                name,
                tokio::spawn(async move { request.send().await.unwrap() }),
            )
        })
        .enumerate();

    let mut progress = Progress::with_style("Trolling", ProgressStyle::Ratio, &config);
    for (i, (name, handle)) in handles {
        progress
            .tick_now(i, request_count, &format!(" {name}"))
            .unwrap();
        let response = handle.await.unwrap();
        config
            .shell()
            .status("Trolling", format!("{name}: {}", response.status()))
            .unwrap();
    }

    config
        .shell()
        .status("Finished", format!("trolling `{email}`"))
        .unwrap();
}
