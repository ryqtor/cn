use tokio::sync::mpsc;
use std::collections::HashSet;
// use rand::seq::SliceRandom; // commented for now to avoid compiling issues if rand isn't perfect

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(100);
    let mut seen_msgs = HashSet::new();
    let peers = vec!["NodeA", "NodeB", "NodeC", "NodeD", "NodeE"];
    
    // random peers select karo
    println!("Handling nodes: {:?}", peers);
}
