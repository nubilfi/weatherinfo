#[cfg(feature = "cli")]
use weatherinfo::{config::ConfigApp, cli_opts::CliOpts, Error};

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = ConfigApp::initialize_app(None)?;

    match tokio::spawn(
        async move {
            CliOpts::parse_opts(&config).await
        }
    ).await
    .unwrap() {
        Ok(_) => Ok(()),
        Err(Error::InvalidInputError(e)) => {
            let msg = CliOpts::api_help_msg();
            println!("Input Error: {e}\n{msg}");
            Ok(())
        }
        Err(e) => Err(e)
    }
}
