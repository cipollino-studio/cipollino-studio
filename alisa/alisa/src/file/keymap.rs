
use std::collections::HashMap;

struct KeyTreeNode {
    children: [u64; 256],
    n_children: u32 
}

impl KeyTreeNode {

    fn empty() -> Self {
        Self {
            children: [0; 256],
            n_children: 0
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for i in 0..256 {
            let byte = i as u8;
            let child = self.children[i];
            if child != 0 {
                result.push(byte);
                result.extend_from_slice(child.to_le_bytes().as_slice());
            }
        }
        result
    }

    fn deserialize(mut data: &[u8]) -> Self {
        let mut result = Self::empty();
        while data.len() >= 9 {
            let byte = data[0];
            let mut child_bytes = [0; 8];
            child_bytes.copy_from_slice(&data[1..9]);
            let child = u64::from_le_bytes(child_bytes);
            data = &data[9..];

            result.children[byte as usize] = child;
            result.n_children += 1;
        }
        result
    }

    fn save(&self, file: &mut verter::File, ptr: u64) {
        let _ = file.write(ptr, &self.serialize());
    }

}

/// A map between object keys and the pointers at which they are stored in the file.
/// This map is also saved in the file, allowing us to efficiently look up where any object is stored without loading the whole file into memory.
pub struct Keymap {
    /// The cached map from object keys to pointers in the Verter file
    map: HashMap<u64, u64>,

    /// Map between `node_ptr`s in the Verter file and the corresponding tree nodes
    nodes: HashMap<u64, KeyTreeNode>,
    /// The pointer to the root tree node 
    root_node_ptr: u64
}

impl Keymap {

    pub fn new(root_ptr: u64) -> Self {
        Self {
            map: HashMap::new(),
            nodes: HashMap::new(),
            root_node_ptr: root_ptr
        }
    }

    pub fn create_empty(file: &mut verter::File) -> Option<(Self, u64)> {
        let root_ptr = file.alloc().ok()?;
        Some((Self::new(root_ptr), root_ptr))
    }

    pub fn ptr(&self) -> u64 {
        self.root_node_ptr
    }

    fn get_node(&mut self, node_ptr: u64, file: &mut verter::File) -> Option<&mut KeyTreeNode> {
        if !self.nodes.contains_key(&node_ptr) {
            let node_data = file.read(node_ptr).ok()?;
            let node = KeyTreeNode::deserialize(&node_data);
            self.nodes.insert(node_ptr, node);
        }
        self.nodes.get_mut(&node_ptr)
    }

    fn set_node_child(&mut self, node_ptr: u64, child_byte: usize, new_child: u64, file: &mut verter::File) -> Option<bool> {
        let root_node = self.root_node_ptr;
        let node = self.get_node(node_ptr, file)?;

        if node.children[child_byte] != 0 {
            node.n_children -= 1;
        }
        if new_child != 0 {
            node.n_children += 1;
        } 
        node.children[child_byte] = new_child;

        // If the node has no more children, delete it
        if node.n_children == 0 && node_ptr != root_node {
            self.nodes.remove(&node_ptr);
            let _ = file.delete(node_ptr);
            return Some(true);
        }       

        node.save(file, node_ptr);
        Some(false)
    }

    fn get_ptr_at_node(&mut self, node_ptr: u64, path: &[u8], file: &mut verter::File) -> Option<u64> {
        let node = self.get_node(node_ptr, file)?;
        let next = path[0] as usize;

        // Create an allocation if necessary
        if node.children[next] == 0 {
            self.set_node_child(node_ptr, next, file.alloc().ok()?, file);
        }

        let node = self.get_node(node_ptr, file)?;
        let child = node.children[next];

        // We're at the leaf node of the tree, so we return the actual object pointer
        if path.len() == 1 {
            return Some(child);
        }

        // Otherwise, go down a layer of the tree
        self.get_ptr_at_node(child, &path[1..], file)
    }

    /// Get the pointer where an object is stored given the object's key.
    /// If the object does not yet have a place in the file, an allocation is made.
    pub fn get_ptr(&mut self, key: u64, file: &mut verter::File) -> Option<u64> {
        if let Some(ptr) = self.map.get(&key) {
            return Some(*ptr);
        } 

        let path = key.to_be_bytes();
        let ptr = self.get_ptr_at_node(self.root_node_ptr, path.as_slice(), file);
        if let Some(ptr) = ptr {
            self.map.insert(key, ptr);
        }
        ptr
    }

    fn delete_at_node(&mut self, node_ptr: u64, path: &[u8], file: &mut verter::File) -> Option<bool> {
        let node = self.get_node(node_ptr, file)?;
        let next = path[0] as usize;

        // If the node has no children past this point, there's nothing to delete
        if node.children[next] == 0 {
            return Some(false);
        }

        let child = node.children[next];

        // We're at the leaf node of the tree, so we delete the actual object data 
        if path.len() == 1 {
            let _ = file.delete(child);
            return self.set_node_child(node_ptr, next, 0, file);
        }

        // Otherwise, go down one layer in the tree
        let did_delete_child = self.delete_at_node(child, &path[1..], file)?;

        // If we deleted the immediate child of this node, remove it from the children of the current node 
        if did_delete_child {
            return self.set_node_child(node_ptr, next, 0, file);
        }

        Some(false)
    }

    /// Delete an object from the file given its key.
    pub fn delete(&mut self, key: u64, file: &mut verter::File) {
        self.map.remove(&key);
        let path = key.to_be_bytes();
        self.delete_at_node(self.root_node_ptr, path.as_slice(), file);
    }

}
