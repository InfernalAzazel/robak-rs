use serde_json::Value;
use std::collections::HashMap;
use yetai_rs::jdy;

#[tokio::main]
async fn main() {
    // let jd = JDSDK::new(
    //     "5dde829086f77b0006f3833e".to_string(),
    //     "60dd8e24224ed700089fbe49".to_string(),
    //     "Q20Prk3r78ih4w0ZYOr6iEFfj9g6cEk0".to_string(),
    // );
    // let arr = jd.get_widgets().await;
    // if let Some(arr) = arr.as_array() {
    //     for x in arr {
    //         println!("测试 {}", x.to_string());
    //     }
    // }

    let cfg = jdy::Config::new(
        "5dde829086f77b0006f3833e".to_string(),
        "60dd8e24224ed700089fbe49".to_string(),
        "Q20Prk3r78ih4w0ZYOr6iEFfj9g6cEk0".to_string(),
    );

    let arr = jdy::get_widgets(cfg).await;
    if let Some(arr) = arr.as_array() {
        for x in arr {
            println!("测试 {}", x.to_string());
        }
    }

    // let arr = jd.get_form_data("", 10, [].to_vec(), Value::from({})).await;
    // if let Some(arr) = arr.as_array(){
    //     for x in arr {
    //         println!("测试2 {}", x.to_string());
    //     }
    // }
    // let arr = jd.get_all_data( [].to_vec(), Value::from({})).await;
    // if let Some(arr) = arr.as_array(){
    //     for x in arr {
    //         println!("测试3 {}", x.to_string());
    //     }
    // }
}
