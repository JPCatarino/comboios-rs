#[tokio::main]
async fn main(){
    let client = comboios::Client::new().await;

    println!("{:?}", client.get_stations().await);
}