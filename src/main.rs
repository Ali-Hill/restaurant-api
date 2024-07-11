use restaurant::configuration::get_configuration;
use restaurant::startup::Application;
use restaurant::telemetry::{get_user, init_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user = get_user("restaurant".into(), "info".into(), std::io::stdout);
    init_user(user);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
