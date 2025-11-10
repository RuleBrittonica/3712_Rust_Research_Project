async fn fetch_and_tag(client: &Client, id: u64) -> Result<String, Error> {
    // EXTRACT START
    let body = client.get(id).await?;     // await inside block
    let tagged = format!("[#{}] {}", id, body);
    // EXTRACT END
    Ok(tagged)
}
