
use std::time::SystemTime;

pub mod fractional_index;
pub mod register;

pub fn time() -> (u64, u64) {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("uh oh, time traveler");
    (time.as_secs(), time.subsec_nanos().into())
}
