use clap::Parser;
use matrix_sdk::{
    Client,
    encryption::{Encryption, identities::UserIdentity, verification::VerificationRequest},
    ruma::UserId,
};
use std::sync::OnceLock;
use test_matrix_rust_sdk::{DEVICE_NAME, cli::CLIArgs, config::Configuration};
use tracing::{Level, error, info};
use tracing_subscriber::filter::EnvFilter;
use url::Url;

static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    // Initialize tracing by tracing-subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::INFO)
        .init();

    let config: Configuration = confy::load_path(args.config_file()).unwrap_or_else(|_| {
        info!("Running this program with default Configuration...");
        Configuration::default()
    });

    CONFIGURATION
        .set(config)
        .expect("Failed to set config to CONFIGURATION");

    let homeserver: Url = CONFIGURATION
        .get()
        .expect("CONFIGURATION is None")
        .homeserver();

    let client: Client = login(homeserver.as_str(), DEVICE_NAME).await?;

    if client.is_active() {
        let user_id: &UserId = client.user_id().ok_or("The UserId is None")?;
        info!("");
        info!("Logged in!!");
        info!("User ID: {}", user_id);

        encrypt(&client).await?;
        mainloop(client).await?;
    } else {
        error!("\nFailed to log in.");
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
            info!("Access this URL to log in: {}", sso_url);
            Ok(())
        })
        .initial_device_display_name(device_name)
        .await?;

    info!("Successed to receive callback. Finished to log in.");

    info!(
        "Logged in as {}, got device_id {} and access_token {}",
        response.user_id, response.device_id, response.access_token,
    );

    Ok(client)
}

async fn mainloop(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn encrypt(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let user_id: &UserId = client.user_id().ok_or("The UserId is None")?;
    let encryption: Encryption = client.encryption();
    let user_identity: UserIdentity = encryption
        .request_user_identity(user_id)
        .await?
        .ok_or("user_identity is None")?;

    user_identity.verify().await?;

    Ok(())
}
