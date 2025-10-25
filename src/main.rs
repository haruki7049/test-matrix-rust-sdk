use matrix_sdk::{Client, ruma::UserId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let homeserver_url: &str = "https://matrix.org";

    let client: Client = Client::builder()
        .homeserver_url(homeserver_url)
        .build()
        .await?;

    let response = client
        .matrix_auth()
        .login_sso(|sso_url| async move {
            // Open sso_url
            println!("{}", sso_url);
            Ok(())
        })
        .initial_device_display_name("test-matrix-rust-sdk")
        .await?;

    println!("Successed to receive callback. Finished to log in.");

    println!(
        "Logged in as {}, got device_id {} and access_token {}",
        response.user_id, response.device_id, response.access_token,
    );

    if client.is_active() {
        let user_id: &UserId = client.user_id().ok_or("The UserId is None")?;
        println!();
        println!("Logged in!!");
        println!("User ID: {}", user_id);
    } else {
        println!("\nFailed to log in.");
    }

    Ok(())
}
