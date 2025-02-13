
#[derive(Clone, Copy)]
struct KeyBlock {
    first: u64,
    last: u64
}

impl KeyBlock {

    fn empty() -> Self {
        Self {
            first: 0,
            last: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.first >= self.last
    }

    fn next_key(&mut self) -> Option<u64> {
        if self.is_empty() {
            return None;
        }
        let key = self.first;
        self.first += 1;
        Some(key)
    }

    fn fill(&mut self, first: u64, last: u64) {
        self.first = first;
        self.last = last;
    }

}

/// A collection of unique keys allocated to a particular client by the server.
pub(crate) struct KeyChain<const N_BLOCKS: usize> {
    /// The blocks of keys on the keychain. We have multiple blocks so that we can continue getting keys if we run out of keys in a particular block
    blocks: [KeyBlock; N_BLOCKS]
}

impl<const N_BLOCKS: usize> KeyChain<N_BLOCKS> {

    pub fn new() -> Self {
        Self {
            blocks: [KeyBlock::empty(); N_BLOCKS]
        }
    }

    pub(crate) fn has_keys(&self) -> bool {
        for block in &self.blocks {
            if !block.is_empty() {
                return true;
            }
        }
        false
    }

    pub(crate) fn wants_keys(&self) -> bool {
        for block in &self.blocks {
            if block.is_empty() {
                return true;
            }
        }
        false
    }

    pub(crate) fn next_key(&mut self) -> Option<u64> {
        for block in &mut self.blocks {
            if let Some(key) = block.next_key() {
                return Some(key);
            }
        }
        None
    }

    pub(crate) fn accept_keys(&mut self, first: u64, last: u64) {
        for block in &mut self.blocks {
            if block.is_empty() {
                block.fill(first, last);
                return;
            }
        }
    }

}
