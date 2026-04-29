use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<String>(32);
    println!("Node discovery channel created");
}
