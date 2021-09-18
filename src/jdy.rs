use crate::request::send_jdy_post;
use reqwest::header::HeaderMap;
use serde_json::json;
use serde_json::Value;

pub struct Config {
    url_get_widgets: String,
    url_get_data: String,
    url_retrieve_data: String,
    url_update_data: String,
    url_create_data: String,
    url_delete_data: String,
    api_key: String,
}
struct Context {
    form_data: Vec<Value>,
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
            url_get_data: set_method("v1".to_string(), "data".to_string()),
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
    cfg: Config,
    data_id: &str,
    limit: i32,
    fields: Vec<String>,
    filter: Value,
) -> Value {
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

// async fn get_next_page(cfg: Config, data_id: &str, fields: Vec<String>, filter: Value) ->  Value{
//     let mut form_data: Vec<Value> = Vec::new();
//     let mx = |data_id| async {
//         let data = get_form_data(cfg, data_id, 100, fields.clone(), filter.clone()).await;
//         if let Some(data) = data.as_array(){
//             for v in data{
//                 form_data.append( &mut vec![]);
//                 let new_data_id = data[data.len() - 1]["_id"].clone();
//                 if let Some(new_data_id) = new_data_id.as_str() {
//                 }
//             }
//         }
//     };
//      json!(form_data.clone())
// }
// pub async fn get_all_data(cfg: Config, fields: Vec<String>, filter: Value) -> Value {
//     get_next_page(cfg,&mut vec![], "", fields, filter).await
// }

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
pub async fn update_data(cfg: Config,  data_id: &str, data: Value) -> Value {
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
