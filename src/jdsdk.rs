use reqwest::header::HeaderMap;
use reqwest::Response;
use serde_json::json;
use serde_json::value::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ptr::null;

pub struct JDSDK {
    url_get_widgets: UrlConfig,
    url_get_data: UrlConfig,
    url_retrieve_data: UrlConfig,
    url_update_data: UrlConfig,
    url_create_data: UrlConfig,
    url_delete_data: UrlConfig,
    api_key: String,
}

#[derive(Clone)]
pub struct UrlConfig {
    v: String,
    app_id: String,
    entry_id: String,
    method: String,
}

impl UrlConfig {
    pub fn set_method(&self, v: String, m: String) -> Self {
        let mut r = self.clone();
        r.v = v;
        r.method = m;
        r
    }
}

impl Display for UrlConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "https://api.jiandaoyun.com/api/{}/app/{}/entry/{}/{}",
             self.v,self.app_id, self.entry_id, self.method
        )
    }
}

impl JDSDK {
    pub fn new(app_id: String, entry_id: String, api_key: String) -> JDSDK {

        let d = UrlConfig {
            v: "".to_string(),
            app_id: app_id.clone(),
            entry_id: entry_id.clone(),
            method: "".to_string(),
        };

        JDSDK {
            url_get_widgets: d.set_method("v1".to_string(),"widgets".to_string()),
            url_get_data: d.set_method("v1".to_string(),"data".to_string()),
            url_retrieve_data: d.set_method("v1".to_string(),"data_retrieve".to_string()),
            url_update_data: d.set_method("v1".to_string(),"data_update".to_string()),
            url_create_data: d.set_method("v1".to_string(),"data_create".to_string()),
            url_delete_data: d.set_method("v1".to_string(),"data_delete".to_string()),
            api_key: api_key.to_string(),
        }
    }

    fn get_req_header(&self) -> HeaderMap {
        // 组装header
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        headers
    }

    pub async fn send_request(
        &self,
        method: String,
        url: String,
        data: Value,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let headers = self.get_req_header();
        // 创建client
        let client = reqwest::Client::new();

        let req = |m| {
            if m == "GET" {
                client.get(url).headers(headers.clone()).query(&data).send()
            } else {
                client.post(url).headers(headers.clone()).json(&data).send()
            }
        };

        let res = req(method).await?;

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

    pub async fn get_widgets(&self) -> Value {
        self.send_request(
            format!("POST"),
            format!("{}", self.url_get_widgets),
            json!({}),
        )
        .await
        .map(|res| json!(res["widgets"]))
        .unwrap_or_default()
    }
    pub async fn get_form_data(&self, data_id: &str, limit: i32, fields: Vec<String>, filter: Value) -> Value {

        self.send_request(
            format!("POST"),
            format!("{}", self.url_get_data),
            json!(
                {
                    "data_id": data_id,
                    "limit": limit,
                    "fields": fields,
                    "filter": filter
                }
            ),
        )
        .await
        .map(|res| json!(res["data"]))
        .unwrap_or_default()

    }
    // pub async fn get_all_data(&self) -> Value {
    //     if let Ok(res) = self.send_request(format!("POST"), format!("{}", self.url_get_widgets), HashMap::new()).await {
    //          json!(res["widgets"])
    //     }else{
    //         json!(null)
    //     }
    //
    // }
    //  pub async fn retrieve_data(&self) -> Value {
    //     if let Ok(res) = self.send_request(format!("POST"), format!("{}", self.url_get_widgets), HashMap::new()).await {
    //          json!(res["widgets"])
    //     }else{
    //         json!(null)
    //     }
    //
    // }
    // pub async fn create_data(&self) -> Value {
    //     if let Ok(res) = self.send_request(format!("POST"), format!("{}", self.url_get_widgets), HashMap::new()).await {
    //          json!(res["widgets"])
    //     }else{
    //         json!(null)
    //     }
    //
    // }
    //  pub async fn update_data(&self) -> Value {
    //     if let Ok(res) = self.send_request(format!("POST"), format!("{}", self.url_get_widgets), HashMap::new()).await {
    //          json!(res["widgets"])
    //     }else{
    //         json!(null)
    //     }
    //
    // }
    //  pub async fn delete_data(&self) -> Value {
    //     if let Ok(res) = self.send_request(format!("POST"), format!("{}", self.url_get_widgets), HashMap::new()).await {
    //          json!(res["widgets"])
    //     }else{
    //         json!(null)
    //     }
    //
    // }
}