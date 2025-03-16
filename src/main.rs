use sha2::{Sha256, Digest};

// // Peerを表す構造体
// #[derive(Debug, Clone)]
// struct Peer {
//     peer_id: [u8; 32],
//     is_registered: bool,
// }

// // コンテンツの構造体
// #[derive(Debug)]
// struct Content {
//     content_id: [u8; 32],
//     data: String,
// }

// XOR距離を求める関数
fn xor_distance(peer_id: &[u8; 32], content_id: &[u8; 32]) -> [u8; 32] {
    let mut dist = [0u8; 32];
    for i in 0..32 {
        dist[i] = peer_id[i] ^content_id[i];
    }
    dist
}
// XOR距離の大小を比較
fn compare_xor_distance(d1: &[u8; 32], d2: &[u8; 32]) -> std::cmp::Ordering {
    d1.cmp(d2)
}

// peer_idの生成
fn generate_peer_id(public_key: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    let result = hasher.finalize();

    let mut peer_id = [0u8; 32];
    peer_id.copy_from_slice(&result);
    peer_id
}

// content_idの作成
fn generate_content_id(content: &str)-> [u8; 32]{
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();

    let mut content_id = [0u8; 32];
    content_id.copy_from_slice(&result);
    content_id
}

fn main(){
    // 事前準備
    let public_key_a = "public_key_A";
    let public_key_b = "public_key_B";
    let content_data = "test";
    // Peeridの生成
    let peer_id_a = generate_peer_id(&public_key_a);
    let peer_id_b = generate_peer_id(&public_key_b);

    let content_id = generate_content_id(&content_data);

    // aの距離
    let distance_a = xor_distance(&peer_id_a, &content_id);
    // bの距離
    let distance_b = xor_distance(&peer_id_b, &content_id);
    match compare_xor_distance(&distance_a, &distance_b) {
        std::cmp::Ordering::Less => println!("Aが近い"),
        std::cmp::Ordering::Equal=> println!("距離が同じ"),
        std::cmp::Ordering::Greater=> println!("Bが近い"),
    }
}