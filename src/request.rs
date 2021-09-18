use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

pub async fn send_jdy_post(
    url: String,
    data: Value,
    headers: HeaderMap,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    // 创建client
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .headers(headers.clone())
        .json(&data)
        .send()
        .await?;

    let status = &res.status();

    let result: HashMap<String, Value> = res.json().await?;

    if status.as_u16() >= 400 {
        if result["code"] == 8303 && true {
            Ok((result))
        } else {
            Ok((HashMap::new()))
        }
    } else {
        Ok((result))
    }
}
