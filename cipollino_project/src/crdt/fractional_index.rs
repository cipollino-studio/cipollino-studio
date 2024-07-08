
/**
    A fractional index(essentially, an arbitrary precision number between 0 and 1) used for implementing LSEQ
 */
#[derive(Clone)]
pub struct FractionalIndex {
    /**
        Stores the bits of the index as booleans, most significan bits first
        Does not store the ones bit, since its always zero
        0.XXXXXX...
     */
    bits: Vec<bool> // TODO: convert to base 2^64 using a list of u64s 
}

impl FractionalIndex {

    /**
        Returns 0.1(binary)
     */
    pub fn half() -> Self {
        Self {
            bits: vec![true]
        }
    }

    /**
        Returns average of self and 0.
        Used to insert an element at the begining of a LSEQ list.
     */
    pub fn avg_with_0(&self) -> Self {
        let mut bits = vec![false];
        bits.extend_from_slice(&self.bits);
        Self {
            bits
        }
    }
    
    /**
        Calculate average of self and 1.
        Used to insert an element at the end of a LSEQ list.
     */
    pub fn avg_with_1(&self) -> Self {
        let mut bits = vec![true];
        bits.extend_from_slice(&self.bits);
        Self {
            bits
        }
    }

    /**
        Calculate the average of 2 fractional indicies.
        Used to insert an element between 2 other elements in an LSEQ list.
     */
    pub fn avg(a: &Self, b: &Self) -> Self {
        let mut bits = vec![false; a.bits.len().max(b.bits.len()) + 1];
        let mut carry = 0;
        for i in (0..(bits.len() - 1)).rev() {
            let a_bit = a.bits.get(i).map(|bit| if *bit { 1 } else { 0 }).unwrap_or(0); 
            let b_bit = b.bits.get(i).map(|bit| if *bit { 1 } else { 0 }).unwrap_or(0); 
            let sum = a_bit + b_bit + carry;
            let bit = sum % 2 == 1;
            carry = sum / 2;
            bits[i + 1] = bit;
        }
        bits[0] = carry == 1;
        Self {
            bits
        }
    }

    /**
        Generate a list of consecutive fractional indexes
     */
    pub fn range(n: usize) -> Vec<Self> {
        if n == 0 {
            return Vec::new();
        }
        if n == 1 {
            return vec![Self::half()]; 
        }

        let n_bits = (n + 1).ilog2();
        (1..=n).map(|i| {
            let bits = (0..=n_bits)
                .map(|idx| {
                    let bit_idx = n_bits - idx;
                    (i & (1 << bit_idx)) > 0
                }) 
                .collect::<Vec<bool>>();

            Self {
                bits
            }
        }).collect()
    }

}

use std::fmt::Debug;
impl Debug for FractionalIndex {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bit_str: String = self.bits.iter().map(|bit| if *bit { '1' } else { '0' }).collect();
        bit_str.fmt(f)
    }

}

impl PartialEq for FractionalIndex {
    
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }

}

impl Eq for FractionalIndex {}

impl PartialOrd for FractionalIndex {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bits.partial_cmp(&other.bits)
    }

}

impl Ord for FractionalIndex {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bits.cmp(&other.bits)
    }

}

impl serde::Serialize for FractionalIndex {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.bits.serialize(serializer)
    }

}

impl<'de> serde::Deserialize<'de> for FractionalIndex {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        Ok(Self {
            bits: Vec::deserialize(deserializer)?
        })
    }
}
