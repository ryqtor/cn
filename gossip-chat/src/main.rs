use tokio::sync::mpsc;
use std::collections::HashSet;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(100);
    let mut seen_msgs = HashSet::new();
    
    // message id set me check karo ki pehle dekha hai ya nahi, warna infinite loop me fass jayenge
    seen_msgs.insert("init_sync".to_string());
}
