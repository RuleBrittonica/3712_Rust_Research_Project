async fn fetch_and_tag(client: &Client, id: u64) -> Result<String, Error> {
    fetch_and_tag_inner(client, id).await
}

async fn fetch_and_tag_inner(client: &Client, id: u64) -> Result<String, Error> {
    let body = client.get(id).await?;
    let tagged = format!("[#{}] {}", id, body);
    Ok(tagged)
}
