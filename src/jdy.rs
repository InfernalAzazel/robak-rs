use reqwest::header::HeaderMap;
use serde_json::json;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::format;


pub struct Config {
    url_get_widgets: String,
    url_get_data: String,
    url_retrieve_data: String,
    url_update_data: String,
    url_create_data: String,
    url_delete_data: String,
    api_key: String,
}

impl Config {
    pub fn new(app_id: String, entry_id: String, api_key: String) -> Config {
        let set_method = |v, method| {
            format!(
                "https://api.jiandaoyun.com/api/{}/app/{}/entry/{}/{}",
                v, app_id, entry_id, method
            )
        };

        Config {
            url_get_widgets: set_method("v1".to_string(), "widgets".to_string()),
            url_get_data: set_method("v2".to_string(), "data".to_string()),
            url_retrieve_data: set_method("v1".to_string(), "data_retrieve".to_string()),
            url_update_data: set_method("v1".to_string(), "data_update".to_string()),
            url_create_data: set_method("v1".to_string(), "data_create".to_string()),
            url_delete_data: set_method("v1".to_string(), "data_delete".to_string()),
            api_key: api_key.to_string(),
        }
    }
    pub fn get_req_header(&self) -> HeaderMap {
        // 组装header
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        headers
    }
}

async fn send_jdy_post(
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
            println!("err: {:?}", result);
            panic!()
        }
    } else {
        Ok((result))
    }
}
pub async fn get_widgets(cfg: Config) -> Value {
    send_jdy_post(
        format!("{}", cfg.url_get_widgets),
        json!({}),
        cfg.get_req_header(),
    )
    .await
    .map(|res| json!(res["widgets"]))
    .unwrap_or_default()
}

pub async fn get_form_data(
    cfg: &Config,
    data_id: &str,
    limit: i32,
    fields: Vec<String>,
    filter: Value,
) -> Value {
    println!("请求的 data_id => {:?}",data_id);
    send_jdy_post(
        format!("{}", cfg.url_get_data),
        json!(
            {
                "data_id": data_id,
                "limit": limit,
                "fields": fields,
                "filter": filter
            }
        ),
        cfg.get_req_header(),
    )
    .await
    .map(|res| json!(res["data"]))
    .unwrap_or_default()
}

pub async fn get_all_data(cfg: Config, fields: Vec<String>, filter: Value) -> Value {
    let mut data_list: Vec<Value> = Vec::new();
    let mut data_id = String::new();
    let mut  data :Value = Value::default();
    loop {

        data = get_form_data(&cfg, &data_id, 2, fields.clone(), filter.clone()).await;
        if let Value::Array(mut data) = data {
                // 获取最后一条数据的id
                match data.last().map(|v| v.as_object()) {
                    None => break,
                    Some(res) => {
                        if let Some(res) = res{
                             data_id = res["_id"].as_str().unwrap().to_string();
                             println!("{:?}", res["msg"]);
                        }
                        data_list.append(&mut data)
                    }
                };
            }
    }
    return json!(data_list);
}

pub async fn retrieve_data(cfg: Config, data_id: &str) -> Value {
    send_jdy_post(
        format!("{}", cfg.url_retrieve_data),
        json!(
            {
                "data_id": data_id,
            }
        ),
        cfg.get_req_header(),
    )
    .await
    .map(|res| json!(res["data"]))
    .unwrap_or_default()
}
pub async fn create_data(cfg: Config, data: Value) -> Value {
    send_jdy_post(
        format!("{}", cfg.url_create_data),
        json!(
            {
                "data": data,
            }
        ),
        cfg.get_req_header(),
    )
    .await
    .map(|res| json!(res["data"]))
    .unwrap_or_default()
}
pub async fn update_data(cfg: Config, data_id: &str, data: Value) -> Value {
    send_jdy_post(
        format!("{}", cfg.url_update_data),
        json!(
            {
                "data_id": data_id,
                "data": data
            }
        ),
        cfg.get_req_header(),
    )
    .await
    .map(|res| json!(res["data"]))
    .unwrap_or_default()
}

pub async fn delete_data(cfg: Config, data_id: &str) -> Value {
    send_jdy_post(
        format!("{}", cfg.url_delete_data),
        json!(
            {
                "data_id": data_id,
            }
        ),
        cfg.get_req_header(),
    )
    .await
    .map(|res| json!(res["data"]))
    .unwrap_or_default()
}
