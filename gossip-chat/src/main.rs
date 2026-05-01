use tokio::sync::mpsc;
use std::collections::HashSet;

// Final G-CHAT
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<(String, String)>(100);
    let mut seen_msgs = HashSet::new();
    let peers = vec!["NodeA", "NodeB", "NodeC", "NodeD", "NodeE"];
    
    tokio::spawn(async move {
        let _ = tx.send(("msg_001".to_string(), "hello".to_string())).await;
    });

    while let Some((msg_id, content)) = rx.recv().await {
        // message id set me check karo ki pehle dekha hai ya nahi, warna infinite loop me fass jayenge
        if seen_msgs.contains(&msg_id) {
            continue;
        }
        seen_msgs.insert(msg_id.clone());
        println!("Received: {}", content);
        
        // 3 random peers ko forward karo
        println!("Forwarding to 3 random peers...");
    }
}
