use reqwest_middleware::ClientWithMiddleware;

pub async fn download(client: &ClientWithMiddleware, url: &str) -> reqwest_middleware::Result<String> {

    let response = client
        .get(url)
        .send()
        .await?;

    let content = response.text().await.unwrap();
    Ok(content)
}