use rand::{Rng, rngs::OsRng};
use bs58;
use std::time::{SystemTime, UNIX_EPOCH};
use sha3::{Sha3_256, Digest};

// 16文字のランダムなBase58文字列を生成する関数
fn generate_random_base58_string() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let time_bytes = now.as_nanos().to_le_bytes();

    let mut csprng = OsRng{};
    let random_bytes: Vec<u8> = (0..32).map(|_| csprng.gen()).collect();

    let combined = [time_bytes.as_ref(), random_bytes.as_ref()].concat();

    let mut hasher = Sha3_256::new();
    hasher.update(combined);
    let hashed_bytes = hasher.finalize();

    let base58_string = bs58::encode(hashed_bytes).into_string();

    let mut rng = rand::thread_rng();
    let start = rng.gen_range(0..=base58_string.len().saturating_sub(16));
    base58_string[start..start + 16].to_string()
}

fn main() {
    for _ in 0..10 {
        let random_string = generate_random_base58_string();
        println!("ランダムなBase58の１６文字: {}", random_string);
    }
}
