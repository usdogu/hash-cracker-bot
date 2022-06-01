use reqwest;
use serde_json::Value;

pub(crate) async fn check_hash(hash: &str) -> String {
    let client = reqwest::ClientBuilder::new()
        .user_agent("MD5 LITE 2.4.3")
        .build()
        .unwrap();

    let resp = client
        .get("https://bluecode.info/md5api/?search[]=".to_owned() + hash)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;
        
    if resp.is_err() {
        return "Make sure that you entered a valid hash.".into();
    }
    let resp = resp.unwrap();
    let hash_result = resp.get(hash);
    if hash_result.is_none() {
        return "Hash not found.".into();
    }
    hash_result.unwrap().as_str().unwrap().into()
}
