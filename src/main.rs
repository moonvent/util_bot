mod apps;

#[tokio::main]
pub async fn main() {
    apps::telegram_bot::initialization::init_bot().await;
}
