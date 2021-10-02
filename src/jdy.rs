use async_recursion::async_recursion;
use core::result::Result;
use reqwest::header::HeaderMap;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

pub struct JDY {
    url_get_widgets: String,
    url_get_data: String,
    url_retrieve_data: String,
    url_update_data: String,
    url_create_data: String,
    url_delete_data: String,
    api_key: String,
}

impl JDY {

    pub fn new(app_id: String, entry_id: String, api_key: String) -> JDY {
        let set_method = |v, method| {
            format!(
                "https://api.jiandaoyun.com/api/{}/app/{}/entry/{}/{}",
                v, app_id, entry_id, method
            )
        };

        JDY {
            url_get_widgets: set_method("v1".to_string(), "widgets".to_string()),
            url_get_data: set_method("v2".to_string(), "data".to_string()),
            url_retrieve_data: set_method("v1".to_string(), "data_retrieve".to_string()),
            url_update_data: set_method("v1".to_string(), "data_update".to_string()),
            url_create_data: set_method("v1".to_string(), "data_create".to_string()),
            url_delete_data: set_method("v1".to_string(), "data_delete".to_string()),
            api_key: api_key.to_string(),
        }
    }

    #[async_recursion]
    async fn post(
        &self,
        url: String,
        data: Value,
    ) -> Result<HashMap<String, Value>, String> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );

        // 创建client
        let client = reqwest::Client::new();
        match client
            .post(url.clone())
            .headers(headers.clone())
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status().clone();
                match resp.json::<HashMap<String, Value>>().await {
                    Ok(v) => {
                        if status.as_u16() >= 400 {
                            if v["code"] == 8303 && true {
                                self.post(url.clone(), data.clone()).await
                            } else {
                                Err(format!("{:?}", v))
                            }
                        } else {
                            Ok(v)
                        }
                    }
                    Err(e) => Err(format!("err: {:?}", e)),
                }
            }
            Err(e) => Err(format!("err: {:?}", e)),
        }
    }

    pub async fn get_widgets(&self) -> Result<Value, String> {
        match self.post(
            format!("{}", self.url_get_widgets),
            json!({}),
        )
        .await
        {
            Ok(v) => Ok(json!(v["widgets"])),
            Err(e) => Err(format!("err: {:?}", e)),
        }
    }

    pub async fn get_form_data(
        &self,
        data_id: &str,
        limit: i32,
        fields: Vec<String>,
        filter: Value,
    ) -> Result<Value, String> {
        match self.post(
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
        {
            Ok(v) => Ok(json!(v["data"])),
            Err(e) => Err(format!("err: {:?}", e)),
        }
    }

    pub async fn get_all_data(&self, fields: Vec<String>, filter: Value) -> Result<Value, String> {
        let mut data_list: Vec<Value> = Vec::new();
        let mut data_id = String::new();

        loop {
            let data = self
                .get_form_data(&data_id, 100, fields.clone(), filter.clone())
                .await?;
            if let Value::Array(mut data) = data {
                // 获取最后一条数据的id
                match data.last().map(|v| v.as_object()) {
                    None => break,
                    Some(res) => {
                        if let Some(res) = res {
                            data_id = res["_id"].as_str().unwrap().to_string();
                            //println!("{:?}", res["msg"]);
                        }
                        data_list.append(&mut data)
                    }
                };
            }
        }
        Ok(json!(data_list))
    }

    pub async fn retrieve_data(&self, data_id: &str) -> Result<Value, String> {
        match self.post(
            format!("{}", self.url_retrieve_data),
            json!(
                {
                    "data_id": data_id,
                }
            ),
        )
        .await
        {
            Ok(v) => Ok(json!(v["data"])),
            Err(e) => Err(format!("err: {:?}", e)),
        }
    }

    pub async fn create_data(&self, data: Value) -> Result<Value, String> {
        match self.post(
            format!("{}", self.url_create_data),
            json!(
                {
                    "data": data,
                }
            ),
        )
        .await
        {
            Ok(v) => Ok(json!(v["data"])),
            Err(e) => Err(format!("err: {:?}", e)),
        }
    }

    pub async fn update_data(&self, data_id: &str, data: Value) -> Result<Value, String> {
        match self.post(
            format!("{}", self.url_update_data),
            json!(
                {
                    "data_id": data_id,
                    "data": data
                }
            ),
        )
        .await
        {
            Ok(v) => Ok(json!(v["data"])),
            Err(e) => Err(format!("err: {:?}", e)),
        }
    }

    pub async fn delete_data(&self, data_id: &str) -> Result<Value, String> {
        match self.post(
            format!("{}", self.url_delete_data),
            json!(
                {
                    "data_id": data_id,
                }
            ),
        )
        .await
        {
            Ok(v) => Ok(json!(v["data"])),
            Err(e) => Err(format!("err: {:?}", e)),
        }
    }
}
