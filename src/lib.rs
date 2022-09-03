use fastrand;
use std::collections::HashMap;

pub struct NUID {
    pre: Vec<u8>,
    seq: i64,
    inc: i64,
}

impl NUID {
    pub fn new() -> NUID {
        let mut result = NUID {
            pre: vec![0; PRE_LEN as usize],
            seq: fastrand::i64(1..MAX_SEQ),
            inc: MIN_INC + fastrand::i64(1..(MAX_INC - MIN_INC)),
        };
        result.randomize_prefix();
        result
    }

    pub fn next(&mut self) -> String {
        self.seq += self.inc;
        if self.seq >= MAX_SEQ {
            self.randomize_prefix();
            self.reset_sequential();
        }

        let mut b: Vec<u8> = vec![0; TOTAL_LEN as usize];
        for (dst, src) in b.iter_mut().zip(&self.pre[..]) {
            *dst = *src;
        }

        let mut i: usize = b.capacity();
        let mut l = self.seq as usize;
        loop {
            if i <= PRE_LEN as usize {
                break;
            }
            i -= 1;
            b[i] = DIGITS_BYTES[l % (BASE as usize)];
            l /= BASE as usize;
        }

        String::from_utf8_lossy(&b).to_string()
    }

    fn reset_sequential(&mut self) {
        self.seq = fastrand::i64(1..MAX_SEQ);
        self.inc = MIN_INC + fastrand::i64(1..(MAX_INC - MIN_INC));
    }

    fn randomize_prefix(&mut self) {
        for idx in 0..PRE_LEN {
            let rnd_idx = (fastrand::u8(..) as i64) % BASE;
            self.pre[idx as usize] = DIGITS_BYTES[rnd_idx as usize];
        }
    }
}

impl std::fmt::Display for NUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.pre, self.seq, self.inc)
    }
}

const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
static DIGITS_BYTES: &[u8] = DIGITS.as_bytes();
const BASE: i64 = 62;
const PRE_LEN: i64 = 12;
const SEQ_LEN: i64 = 10;
const MAX_SEQ: i64 = 839299365868340224; // base^seqLen == 62^10
const MIN_INC: i64 = 33;
const MAX_INC: i64 = 333;
const TOTAL_LEN: i64 = PRE_LEN + SEQ_LEN;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(DIGITS.len(), BASE as usize);
    }

    #[test]
    fn test_nuid_rollover() {
        let mut sut = NUID::new();
        sut.seq = MAX_SEQ;
        let old_pre = sut.pre.clone();

        sut.next();

        assert_ne!(sut.pre, old_pre);
    }

    #[test]
    fn test_guid_len() {
        let mut sut = NUID::new();

        let id = sut.next();

        assert_eq!(id.len(), TOTAL_LEN as usize);
    }

    #[test]
    fn test_proper_prefix() {
        let mut min: u8 = 255;
        let mut max: u8 = 0;
        for d in DIGITS_BYTES {
            if *d < min {
                min = *d;
            }
            if *d > max {
                max = *d;
            }
        }
        let total = 100000;
        for _ in 0..total {
            let sut = NUID::new();
            for d in sut.pre {
                assert!(d >= min);
                assert!(d <= max);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_basic_uniqueness() {
        let n = 10000000;
        let mut map: HashMap<String, u8> = HashMap::new();
        let mut sut = NUID::new();

        for _ in 0..n {
            let id = sut.next();
            assert_eq!(None, map.insert(id, 1));
        }
    }
}
