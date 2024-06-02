use nasa_download::iterate_table;
use tokio;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    iterate_table().await
}
