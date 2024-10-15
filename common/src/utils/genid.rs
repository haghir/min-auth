use chrono::Utc;
use log::info;
use rand::{thread_rng, Rng};

use super::base35::to_base35;
use super::threadid::gettid;

pub fn genid() -> String {
    // Return the base35 string from the following byte sequence.
    //
    // |XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX XX |
    // |--------------------|-----------|---------------------|
    // |   Random Number    | Thread ID | Timestamp (micros)  |
    // |     (7 bytes)      | (4 bytes) |      (7 bytes)      |
    //
    // The leading two letters of the string will be removed
    // because only few characters (14 - 1a) can be it.

    let now = Utc::now().timestamp_micros();
    let tid = gettid();
    let mut rng = thread_rng();
    let mut val: Vec<u8> = (0..18)
        .map(|i| {
            if i == 0 {
                // Make the number 29-digit in 35 decimal notation.
                rng.gen::<u8>() | 0xe0
            } else if i < 7 {
                rng.gen()
            } else if i < 11 {
                (tid >> ((10 - i) * 8)) as u8
            } else {
                (now >> ((17 - i) * 8)) as u8
            }
        })
        .collect();
    val[0] |= 0x80;

    let mut ret = to_base35(&val);
    let removed1 = ret.remove(0);
    let removed2 = ret.remove(0);
    info!("New ID: ({}{}){}", removed1, removed2, ret);
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
                assert_eq!(ids[i].len(), 27);
                assert_eq!(ids[j].len(), 27);
                assert_ne!(ids[i], ids[j]);
            }
        }
    }
}
