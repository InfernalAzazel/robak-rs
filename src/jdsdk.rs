use reqwest::header::HeaderMap;
use reqwest::Response;
use serde_json::json;
use serde_json::value::Value;
use std::collections::HashMap;

pub struct JDSDK {
    url_get_widgets: String,
    url_get_data: String,
    url_retrieve_data: String,
    url_update_data: String,
    url_create_data: String,
    url_delete_data: String,
    api_key: String,
}

impl JDSDK {
    pub fn new(app_id: String, entry_id: String, api_key: String) -> JDSDK {
        let url_bsite = "https://api.jiandaoyun.com";
        JDSDK {
            url_get_widgets: format!(
                "{}/api/v1/app/{}/entry/{}/widgets",
                url_bsite, app_id, entry_id
            ),
            url_get_data: format!(
                "{}/api/v1/app/{}/entry/{}/data",
                url_bsite, app_id, entry_id
            ),
            url_retrieve_data: format!(
                "{}/api/v1/app/{}/entry/{}/data_retrieve",
                url_bsite, app_id, entry_id
            ),
            url_update_data: format!(
                "{}/api/v1/app/{}/entry/{}/data_update",
                url_bsite, app_id, entry_id
            ),
            url_create_data: format!(
                "{}/api/v1/app/{}/entry/{}/data_create",
                url_bsite, app_id, entry_id
            ),
            url_delete_data: format!(
                "{}/api/v1/app/{}/entry/{}/data_delete",
                url_bsite, app_id, entry_id
            ),
            api_key: format!("{}", api_key),
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
        data: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let headers = self.get_req_header();
        // 请求要创建client
        let client = reqwest::Client::new();

        let req = |m| if m == "GET" {
            client.get(url).headers(headers.clone()).query(&data).send()
        } else {
            client.post(url).headers(headers.clone()).json(&data).send()
        };

        let res = req(method).await?;

        let status = &res.status();

        let result: HashMap<String, Value> =  res.json().await?;

         if status.as_u16() >= 400 {
            if result["code"] == 8303 && true {
                Ok((result))
            }else{
               Ok((HashMap::new()))
            }
         } else{
             Ok((result))
         }


        //let Ok(result) = Ok(res.json().await?);
        //println!("status {:?}", status);
        //println!("json {:?}", result);


       // Ok((HashMap::new()))

    }

    pub async fn get_widgets(&self) -> Value {
        if let Ok(res) = self.send_request(format!("POST"), format!("{}", self.url_get_widgets), HashMap::new()).await {
             json!(res["widgets"])
        }else{
            json!(null)
        }

    }
}