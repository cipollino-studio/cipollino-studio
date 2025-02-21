
use std::hash::Hash;

pub fn hash<H: Hash>(x: &H) -> u64 {
    ahash::RandomState::with_seeds(1, 9, 8, 4).hash_one(x)
}
