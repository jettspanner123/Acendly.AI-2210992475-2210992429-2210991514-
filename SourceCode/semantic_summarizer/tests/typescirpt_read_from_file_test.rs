#[tokio::test]
async fn test_typescript_parse_from_file() {
    // Read sample.ts relative to the project root (where cargo test is run from)
    let source_code = std::fs::read_to_string("sample.ts")
        .expect("Failed to read sample.ts — make sure the server is run from the semantic_summarizer directory");

    let body = serde_json::json!({
        "source_code": source_code,
        "language": "typescript"
    });

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/ast/parse")
        .json(&body)
        .send()
        .await
        .expect("Failed to send request — is the server running on localhost:8080?");

    let status = response.status();
    let response_text = response.text().await.expect("Failed to read response body");

    println!("\n========== RESPONSE ==========");
    println!("Status: {}", status);
    println!("Body:\n{}", response_text);
    println!("==============================\n");

    assert!(status.is_success(), "Expected 2xx status, got: {}", status);
}
