use chrono::Utc;
use log::info;
use rand::{thread_rng, Rng};

use crate::base58::to_base58;
use crate::threadid::gettid;

pub fn genid() -> String {
    // Returns the base58 string from the following byte sequence.
    //
    // |XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX |
    // |--------------------|-----------|---------------------|
    // |   Random Number    | Thread ID | Timestamp (micros)  |
    // |     (7 bytes)      | (4 bytes) |      (7 bytes)      |
    //
    // The leading letter of the string will be removed
    // because only few character can be it.

    let now = Utc::now().timestamp_micros();
    let tid = gettid();
    let mut rng = thread_rng();
    let mut val: Vec<u8> = (0..18).map(|i| if i < 7 {
        rng.gen()
    } else if i < 11 {
        (tid >> ((10 - i) * 8)) as u8
    } else {
        (now >> ((17 - i) * 8)) as u8
    }).collect();
    val[0] |= 0x80;

    let mut ret = to_base58(&val);
    let removed = ret.remove(0);
    info!("New ID: ({}){}", removed, ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn genid_test() {
        let sz = 100;
        let ids: Vec<String> = (0..sz).map(|_| genid()).collect();
        for i in 1..sz {
            for j in 0..i {
                assert_eq!(ids[i].len(), 24);
                assert_eq!(ids[j].len(), 24);
                assert_ne!(ids[i], ids[j]);
            }
        }
    }
}
