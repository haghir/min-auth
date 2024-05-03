use itertools::Itertools;

const CHR: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// based on https://github.com/trezor/trezor-crypto/blob/master/base58.c
pub(crate) fn to_base58(data: &[u8]) -> String {
    let base = CHR.len() as u32;
    let zcount = data.iter().take_while(|x| **x == 0).count();
    let slen = data.len();
    let dlen = (slen - zcount) * 137 / 100 + 1;
    let mut buf = vec![0u8; dlen];

    let mut i = zcount;
    let mut h = 0;
    while i < slen {
        let mut carry = data[i] as u32;
        let mut j = 0;

        while j < h || carry != 0 {
            carry += 256 * buf[j] as u32;
            buf[j] = (carry % base) as u8;
            carry /= base;
            j += 1;
        }

        i += 1;
        h = j;
    }

    let mut ret = (0..zcount).map(|_| "1").join("");
    for i in (dlen - h)..dlen {
        ret.push(CHR[buf[dlen - i - 1] as usize] as char);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        test_body(9761452456310830576 , "PfCnNX9dnzK");
        test_body(12786007138795231881, "WgQUdHfrEUG");
        test_body(769922394487803181  , "2nf4EkCWY5J");
        test_body(584312613703605355  , "2MfhEYTSk1x");
        test_body(10774238967919020214, "S1ZHCoByVkh");
        test_body(7709672512753955694 , "Jty9YcHrBfb");
        test_body(10567243695299174083, "RXgvfzEZzQS");
        test_body(14428516099680356142, "aVYFtV9a7VP");
        test_body(796231214057025354  , "2rCVYpjXETb");
        test_body(2430107206234966077 , "6eArvaAoWv8");
    }

    fn test_body(val: u64, expected: &str) {
        let mut arr: Vec<u8> = (0..8).map(|i| (val >> ((7 - i) * 8)) as u8).collect();
        let mut expected = expected.to_string();
        assert_eq!(expected, to_base58(&arr));

        for _ in 0..10 {
            arr.insert(0, 0);
            expected.insert(0, '1');
            assert_eq!(expected, to_base58(&arr));
        }
    }
}
