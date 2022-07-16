use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Url};

pub fn get_requests(
    client: &Client,
    email: &str,
    first_name: &str,
    last_name: &str,
) -> Vec<(&'static str, RequestBuilder)> {
    Vec::from([
        (
            "Kobieca Foto Szkoła",
            mailerlite(
                client,
                "c2m5l1",
                email,
                format!("{first_name} {last_name}").as_str(),
            ),
        ),
        (
            "Daję Słowo",
            mailerlite(
                client,
                "h9j6v0",
                email,
                format!("{first_name} {last_name}").as_str(),
            ),
        ),
        (
            "Monika Liga",
            mailerlite(
                client,
                "u6c6u8",
                email,
                format!("{first_name} {last_name}").as_str(),
            ),
        ),
        (
            "piesologia",
            mailerlite(
                client,
                "r7z4a9",
                email,
                format!("{first_name} {last_name}").as_str(),
            ),
        ),
        (
            "Google Newsletter",
            client
                .post("https://services.google.com/fb/submissions/thekeywordnewsletterprod/")
                .form(&HashMap::from([
                    ("EmailAddress", email),
                    ("LanguagePreference", "en-US"),
                    ("TheKeywordNews", "UNCONFIRMED"),
                ])),
        ),
    ])
}

fn mailerlite(client: &Client, form_id: &str, email: &str, name: &str) -> RequestBuilder {
    client.get(
        Url::parse_with_params(
            &format!("https://app.mailerlite.com/webforms/submit/{form_id}"),
            [
                ("fields[name]", name),
                ("fields[email]", email),
                ("ml-submit", "1"),
                ("ajax", "1"),
            ],
        )
        .unwrap(),
    )
}
