use matrix_sdk::{Client, ruma::UserId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let homeserver_url: &str = "https://matrix.org";
    let device_name: &str = "test-matrix-rust-sdk";

    let client: Client = login(homeserver_url, device_name).await?;

    if client.is_active() {
        let user_id: &UserId = client.user_id().ok_or("The UserId is None")?;
        println!();
        println!("Logged in!!");
        println!("User ID: {}", user_id);

        encrypt(&client).await?;
        mainloop(client).await?;
    } else {
        println!("\nFailed to log in.");
    }

    Ok(())
}

async fn login(
    homeserver_url: &str,
    device_name: &str,
) -> Result<Client, Box<dyn std::error::Error>> {
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
        .initial_device_display_name(device_name)
        .await?;

    println!("Successed to receive callback. Finished to log in.");

    println!(
        "Logged in as {}, got device_id {} and access_token {}",
        response.user_id, response.device_id, response.access_token,
    );

    Ok(client)
}

async fn mainloop(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn encrypt(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
