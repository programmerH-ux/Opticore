use sha2::{Sha256, Digest};
use std::env;

const VALID_HASH: &str = "5df466b29ac8aa660716e60f646955ad69b17e5a6bdc08566488586b921b7ba3  -";

pub fn ensure_pro() {
    let key = env::var("OPTIENGINE_LICENSE")
        .expect("No license key found. Set OPTIENGINE_LICENSE environment variable.");

    let mut hasher = Sha256::new();
    hasher.update(key);
    let result = format!("{:x}", hasher.finalize());

    if result != VALID_HASH {
        panic!("Invalid OptiEngine Pro license key.");
    }
}
