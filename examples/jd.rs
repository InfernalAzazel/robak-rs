use yetai_rs::jdsdk::JDSDK;

#[tokio::main]
async fn main() {
    let jd = JDSDK::new(
        "5dde829086f77b0006f3833e".to_string(),
        "60dd8e24224ed700089fbe49".to_string(),
        "Q20Prk3r78ih4w0ZYOr6iEFfj9g6cEk0".to_string(),
    );
    let arr = jd.get_widgets().await;
    for i in 0..arr.as_array().unwrap().len() {
        println!("测试 {}", arr.as_array().unwrap()[i]);
    }

}
