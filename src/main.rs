mod error;
mod config;
mod ais;

#[tokio::main]
async fn main() {
    let oa_client = ais::new_oa_client();
    println!("{oa_client:?}");
}
