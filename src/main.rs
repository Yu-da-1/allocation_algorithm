use sha2::{Sha256, Digest};
use k256::{ecdsa::{SigningKey, VerifyingKey}, elliptic_curve::rand_core::OsRng};
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
fn generate_peer_id(public_key: &[u8]) -> [u8; 32] {
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

// 鍵の生成
fn generate_key() -> (SigningKey, VerifyingKey) {
    let secret_key = SigningKey::random(&mut OsRng);
    let public_key = VerifyingKey::from(&secret_key);
    (secret_key, public_key)
}

fn main(){
    // 複数のpeerの作成
    let mut peers = Vec::new();
    for _ in 0..10 {
        let (_, public_key) = generate_key();
        let peer_id = generate_peer_id(&public_key.to_encoded_point(false).as_bytes());
        peers.push(peer_id);
    }

    // コンテンツの作成
    let content_data = "test";
    let content_id = generate_content_id(&content_data);

    // Peerの距離の計算を行いソートする
    let mut peer_distances: Vec<([u8; 32], [u8; 32])> = peers
    .into_iter()
    .map(|peer_id| {
        let dist = xor_distance(&peer_id, &content_id);
        (peer_id, dist)
    })
    .collect();

    // ソート時、XOR結果が小さい順(=近い順)になるよう比較
    peer_distances.sort_by(|(_, d1), (_, d2)| d1.cmp(d2));

    println!("--- Top nearest peers ---");
    for (i, (peer_id, _)) in peer_distances.iter().take(3).enumerate() {
        println!("Rank {}: PeerID ={:?}", i+1, peer_id);
    }
}