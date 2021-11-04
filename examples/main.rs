use serde_json::json;
use yetai_rs::jdy;
use core::result::Result;

#[tokio::main]
async fn main() {
    // 输入自己的key
    let form = jdy::JDY::new(
        "".to_string(),
        "".to_string(),
        "".to_string(),
    );

    match form.get_widgets().await {
        Ok(v) => {
            if let Some(v) = v.as_array() {
                for x in v {
                    println!("测试 {}", x.to_string());
                }
            }
        }
        Err(e) => {
            println!("{}", e)
        }
    }
    for _ in 0..2 {
        match form.get_all_data(vec![], json!({})).await {
            Ok(v) => {
                if let Some(v) = v.as_array() {
                    for x in v {
                        println!("测试 {}", x.to_string());
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    let res = match form.create_data( json!({
        "msg":{"value":"99999999999"}
    })).await {
         Ok(v) => {
             v
         }
         Err(e) => {
             println!("{}", e);

         }

     };
    println!("{:?}", res)

}

async fn business() {
    let form = jdy::JDY::new(
        "5dde829086f77b0006f3833e".to_string(),
        "60dd8e24224ed700089fbe49".to_string(),
        "Q20Prk3r78ih4w0ZYOr6iEFfj9g6cEk0".to_string(),
    );

    res = form.get_widgets().await?
}
