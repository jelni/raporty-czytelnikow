use std::env;
use std::time::Duration;

use reqwest::Client;

const ID_MARKER: &str = "data-raport--item-id-value=\"";
const URL_MARKER: &str = "<a rel=\"nofollow\" href=\"";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().ok();

    let telegram_token = env::var("TELEGRAM_TOKEN").unwrap();
    let chat_id = env::var("TELEGRAM_CHAT_ID").unwrap();

    let client = Client::new();
    let mut last_id = None;

    loop {
        let response = client
            .get("https://www.trojmiasto.pl/raport/ajax/ReportList/?tags=120")
            .send()
            .await
            .unwrap();

        let html = response.text().await.unwrap();

        let reports = html
            .rmatch_indices(ID_MARKER)
            .map(|(index, _)| {
                let date_start_index = index + ID_MARKER.len();

                let date_end_index = html[date_start_index..].find('"').unwrap() + date_start_index;

                let url_start_index = html[date_end_index..].find(URL_MARKER).unwrap()
                    + URL_MARKER.len()
                    + date_end_index;

                let url_end_index = html[url_start_index..].find('"').unwrap() + url_start_index;

                (
                    &html[url_start_index..url_end_index],
                    html[date_start_index..date_end_index]
                        .parse::<u32>()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        for &(report_url, report_id) in &reports {
            if last_id.is_some_and(|last_id| report_id > last_id) {
                client
                    .post(format!(
                        "https://api.telegram.org/bot{telegram_token}/sendMessage"
                    ))
                    .form(&[("chat_id", chat_id.as_str()), ("text", report_url)])
                    .send()
                    .await
                    .unwrap();
            }
        }

        if let Some(highest_report_id) = reports.into_iter().map(|(_, report_id)| report_id).max() {
            last_id = Some(highest_report_id);
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
