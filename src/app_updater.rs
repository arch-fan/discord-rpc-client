use reqwest;
use serde_json::json;

pub async fn update(app_id: &String, auth_header: &String, app_name: &String) -> Result<(), Box<dyn std::error::Error>> {

    let url = format!("https://discord.com/api/v9/applications/{app_id}");

    let client = reqwest::Client::new();
    
    let request = client.patch(&url)
        .json(&json!({
            "name": app_name
        }))
        .header("Authorization", auth_header)
        .send()
        .await?;

    let req_status = request.status();

    if req_status != 200 {
        eprintln!("An error has ocourred when updating app name. Try updating Auth header or changing app name length. {req_status}")
    }

    Ok(())
}
