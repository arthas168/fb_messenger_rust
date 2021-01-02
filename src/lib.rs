pub async fn send_message(
    message: String,
    access_token: String,
    messaging_type: String,
    recipient_object: String,
) -> () {
    let client = reqwest::Client::new();
    let url = "https://graph.facebook.com/v9.0/me/messages";
    let params = [
        ("access_token", &access_token),
        ("messaging_type", &messaging_type),
        ("recipient", &recipient_object),
        ("message", &message),
    ];

    client
        .post(url)
        .form(&params)
        .send()
        .await
        .expect("Failed to send message.");
}
